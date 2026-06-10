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
    getCodamaVersionPageFragment,
    getEnumPageFragment,
    getLiteralUnionPageFragment,
    getModPagesRenderMap,
    getNodePageFragment,
    getPageFragment,
    getRegisteredUnionPageFragment,
    getUnionPageFragment,
} from './fragments';
import { getReferencedLiteralUnions } from './literalUnions';
import { isNestableNode } from './nodeFlavor';
import {
    buildRenderScope,
    type GenerateOptions,
    type RenderOptions,
    type RenderScope,
    validateRenderOptions,
} from './options';
import { getRepoDirectory } from './repoDirectory';
import { getEmittableUnions, isRegisteredCategoryUnion } from './unions';

export { CATEGORY_DIRECTORIES, type CategoryRouting, CATEGORY_ROUTING } from './defaults';
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
 * Build the render map: one page per node, per emittable union, per
 * enumeration, and per literalUnion value-set, plus a
 * `codama_version.rs` constant pinned to the spec version.
 */
function getSpecPagesRenderMap(spec: Spec, scope: RenderScope): RenderMap<Fragment> {
    const entries: Record<Path, Fragment> = {};

    for (const category of spec.categories) {
        const folder = scope.categoryDirectories.get(category.name);

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
            // Nestable type nodes (members of `nestedTypeNode.wrappers`)
            // are generic over `<T: TypeNodeUnionTrait>` with bespoke
            // nest/un-nest conversions and `NestedTypeNodeTrait`
            // impls. They stay hand-written in v1.
            if (isNestableNode(node.kind, spec)) continue;
            const path = joinPath(folder, `${snakeCase(node.kind)}.rs`);
            entries[path] = getPageFragment(getNodePageFragment(node, routing, spec));
        }
        // `emitUnions: false` keeps a category's unions entirely
        // hand-written (currently `type`, whose four unions are
        // generic / cross-category / have bespoke `TryFrom` bridges).
        if (routing.emitUnions === false) continue;
        for (const union of getEmittableUnions(category, spec)) {
            const path = joinPath(folder, `${snakeCase(union.name)}.rs`);
            const fragment = isRegisteredCategoryUnion(union, spec)
                ? getRegisteredUnionPageFragment(union, spec)
                : getUnionPageFragment(union, spec);
            entries[path] = getPageFragment(fragment);
        }
    }

    // `literalUnion` TypeExprs are anonymous and inline; discover
    // them by walking every node attribute, dedup by value-set, and
    // emit one shell per distinct value-set into `shared/`. The
    // bespoke serde/From/Default impls stay hand-written alongside.
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

    // `CODAMA_VERSION` constant pinned to the spec version at
    // generation time, mirroring the JS `CODAMA_VERSION`. Used by the
    // hand-written `RootNode::default()` to tag the document.
    entries['codama_version.rs'] = getPageFragment(getCodamaVersionPageFragment(spec.version));

    return createRenderMap(entries);
}
