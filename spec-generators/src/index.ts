import {
    createRenderMap,
    deleteDirectory,
    joinPath,
    mergeRenderMaps,
    type Path,
    type RenderMap,
    snakeCase,
    writeRenderMap,
} from '@codama/fragments';
import { type Fragment } from '@codama/fragments/rust';
import { getSpec, type Spec } from '@codama/spec';

import { CATEGORY_ROUTING } from './defaults';
import {
    getEnumPageFragment,
    getLiteralUnionPageFragment,
    getModPagesRenderMap,
    getNodePageFragment,
    getPageFragment,
    getRegisteredUnionPageFragment,
    getUnionPageFragment,
} from './fragments';
import { getReferencedLiteralUnions } from './literalUnions';
import {
    buildRenderScope,
    type GenerateOptions,
    type RenderOptions,
    type RenderScope,
    validateRenderOptions,
} from './options';
import { getRepoDirectory } from './repoDirectory';
import { getEmittableUnions, isRegisteredCategoryUnion } from './unions';

export { CATEGORY_DIRECTORIES, type CategoryRouting, CATEGORY_ROUTING, FIELD_TYPE_OVERRIDES } from './defaults';
export {
    buildRenderScope,
    type GenerateOptions,
    type RenderOptions,
    type RenderScope,
    type ResolvedRenderOptions,
    validateRenderOptions,
} from './options';

export interface GenerateResult {
    /** The output directory the generator wrote to. */
    readonly outputDir: Path;
}

/**
 * Run the generator against the embedded `@codama/spec` and write the
 * full `codama-nodes/src/generated/` tree. The output directory is
 * wiped before each run so stale files cannot survive.
 */
export function generate(): GenerateResult {
    const spec = getSpec();
    const outputDir = joinPath(getRepoDirectory(), 'codama-nodes', 'src', 'generated');
    generateInto(spec, { outputDir, targetSpecMajor: 1 });
    return { outputDir };
}

/**
 * Build the render map and write it to disk under `options.outputDir`.
 * The target directory is wiped before each run so stale files cannot
 * survive. No formatter is applied — chain `cargo fmt` afterwards.
 */
export function generateInto(spec: Spec, options: GenerateOptions): void {
    const renderMap = getRenderMap(spec, options);
    deleteDirectory(options.outputDir);
    writeRenderMap(renderMap, options.outputDir);
}

/**
 * Pure-and-sync render-map entry point. Tests can call this directly
 * without touching the filesystem and assert against individual
 * entries via `getFromRenderMap`.
 */
export function getRenderMap(spec: Spec, options: RenderOptions): RenderMap<Fragment> {
    validateRenderOptions(spec, options);
    const scope = buildRenderScope(options);
    const specPages = getSpecPagesRenderMap(spec, scope);
    const modPages = getModPagesRenderMap(specPages);
    return mergeRenderMaps([specPages, modPages]);
}

/**
 * Walk every spec category covered by {@link CATEGORY_ROUTING} and
 * emit one page per node and one page per emittable union, plus one
 * page per enumeration in every category that declares them (which is
 * orthogonal to routing — enumerations live wherever the spec puts
 * them, and today that's the `shared` category). After the category
 * loop, emit one page per distinct `literalUnion` value-set found
 * anywhere in the spec; these target `shared/` because they're
 * cross-category infrastructure with no native home in the spec's
 * category model. Returns a render map keyed by output path (relative
 * to `generated/`) with the resolved page fragment as the value.
 */
function getSpecPagesRenderMap(spec: Spec, scope: RenderScope): RenderMap<Fragment> {
    const entries: Record<Path, Fragment> = {};

    for (const category of spec.categories) {
        const folder = scope.categoryDirectories.get(category.name);

        // Enumerations are emitted whenever a category declares them,
        // independent of whether the category's nodes/unions are
        // generated yet. The directory must still be configured.
        if (category.enumerations.length > 0) {
            if (folder === undefined) {
                throw new Error(
                    `categoryDirectories has no entry for category "${category.name}" (which declares enumerations).`,
                );
            }
            for (const enumeration of category.enumerations) {
                const path = joinPath(folder, `${snakeCase(enumeration.name)}.rs`);
                entries[path] = getPageFragment(getEnumPageFragment(enumeration));
            }
        }

        const routing = CATEGORY_ROUTING.get(category.name);
        if (!routing) continue;
        if (folder === undefined) {
            throw new Error(`categoryDirectories has no entry for category "${category.name}".`);
        }

        for (const node of category.nodes) {
            const path = joinPath(folder, `${snakeCase(node.kind)}.rs`);
            entries[path] = getPageFragment(getNodePageFragment(node, routing));
        }
        for (const union of getEmittableUnions(category, spec)) {
            // The on-disk file name follows the spec union name in
            // snake_case (e.g. `linkNode` → `link_node.rs`).
            const path = joinPath(folder, `${snakeCase(union.name)}.rs`);
            // Category unions whose `registered<X>` twin has extra
            // `#[registered]`-only members (currently `value`, and in
            // future `type` / `contextualValue`) are emitted via the
            // `RegisteredNodes` derive; the standalone twin is then
            // auto-derived. Other unions take the plain `#[node_union]`
            // path.
            const fragment = isRegisteredCategoryUnion(union, spec)
                ? getRegisteredUnionPageFragment(union, spec)
                : getUnionPageFragment(union, spec);
            entries[path] = getPageFragment(fragment);
        }
    }

    // `literalUnion` TypeExprs are anonymous and inline in v1 — they
    // don't belong to any category's registry. Discover them by
    // walking every node attribute spec-wide, dedup by value-set, and
    // emit one Rust enum shell per distinct value-set into `shared/`.
    // The bespoke `Serialize`/`Deserialize`/`From<bool>`/`Default` impls
    // stay hand-written in the companion file alongside the shell.
    const literalUnions = getReferencedLiteralUnions(spec);
    if (literalUnions.length > 0) {
        const sharedFolder = scope.categoryDirectories.get('shared');
        if (sharedFolder === undefined) {
            throw new Error(
                'categoryDirectories has no entry for "shared", which is the destination for generated literalUnion shells.',
            );
        }
        for (const ref of literalUnions) {
            const path = joinPath(sharedFolder, `${snakeCase(ref.typeName)}.rs`);
            entries[path] = getPageFragment(getLiteralUnionPageFragment(ref));
        }
    }

    return createRenderMap(entries);
}
