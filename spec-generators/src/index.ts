import {
    createRenderMap,
    deleteDirectory,
    joinPath,
    mergeRenderMaps,
    pascalCase,
    type Path,
    type RenderMap,
    snakeCase,
    writeRenderMap,
} from '@codama/fragments';
import { type Fragment } from '@codama/fragments/rust';
import { getSpec, type Spec } from '@codama/spec';

import { CATEGORY_ROUTING, UNION_NAME_OVERRIDES } from './defaults';
import { getModPagesRenderMap, getNodePageFragment, getPageFragment, getUnionPageFragment } from './fragments';
import {
    buildRenderScope,
    type GenerateOptions,
    type RenderOptions,
    type RenderScope,
    validateRenderOptions,
} from './options';
import { getRepoDirectory } from './repoDirectory';
import { getEmittableUnions } from './unions';

export {
    CATEGORY_DIRECTORIES,
    type CategoryRouting,
    CATEGORY_ROUTING,
    ENUMERATION_NAME_OVERRIDES,
    FIELD_TYPE_OVERRIDES,
    HAND_WRITTEN_UNIONS,
    type InlineUnionConfig,
    INLINE_UNIONS,
    UNION_NAME_OVERRIDES,
} from './defaults';
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
 * emit one page per node and one page per emittable union. Returns a
 * render map keyed by output path (relative to `generated/`) with the
 * resolved page fragment as the value.
 */
function getSpecPagesRenderMap(spec: Spec, scope: RenderScope): RenderMap<Fragment> {
    const entries: Record<Path, Fragment> = {};

    for (const category of spec.categories) {
        const routing = CATEGORY_ROUTING.get(category.name);
        if (!routing) continue;
        const folder = scope.categoryDirectories.get(category.name);
        if (folder === undefined) {
            throw new Error(`categoryDirectories has no entry for category "${category.name}".`);
        }

        for (const node of category.nodes) {
            const path = joinPath(folder, `${snakeCase(node.kind)}.rs`);
            entries[path] = getPageFragment(getNodePageFragment(node, routing));
        }
        for (const union of getEmittableUnions(category)) {
            // The on-disk file name follows the *Rust* enum name in
            // snake_case (e.g. `linkNode` → `link_node.rs`), respecting
            // any {@link UNION_NAME_OVERRIDES}. Honouring the override
            // keeps the file name equal to the Rust type name (e.g.
            // `enumValuePayload` → `EnumVariantData` →
            // `enum_variant_data.rs`).
            const rustName = UNION_NAME_OVERRIDES.get(union.name) ?? pascalCase(union.name);
            const path = joinPath(folder, `${snakeCase(rustName)}.rs`);
            entries[path] = getPageFragment(getUnionPageFragment(union, spec));
        }
    }

    return createRenderMap(entries);
}
