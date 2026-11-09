/**
 * Default options for the spec v2 generator.
 */

/**
 * Generation options for a spec category whose nodes the generator emits.
 *
 * `emitUnions` defaults to `true`; set to `false` for categories whose
 * unions are too bespoke for the standard renderers (currently `type`).
 */
export type CategoryOptions = { readonly emitUnions?: boolean };

/**
 * The spec categories whose nodes the generator emits. Every emitted node
 * is a direct variant of the generated `Node` and `NodeRef` enums.
 * Categories absent from this map are not generated.
 */
export const CATEGORY_OPTIONS: ReadonlyMap<string, CategoryOptions> = new Map([
    ['contextualValue', {}],
    ['count', {}],
    ['discriminator', {}],
    ['display', {}],
    ['link', {}],
    ['pdaSeed', {}],
    ['topLevel', {}],
    ['transform', {}],
    ['type', { emitUnions: false }],
    ['value', {}],
]);

/**
 * Mapping from spec category name to the output subdirectory the
 * generator emits its entities into (relative to `generated/`). The
 * empty string places `topLevel` entities at the root.
 *
 * Only categories with an entry in {@link CATEGORY_OPTIONS} get node
 * and union pages. `shared` holds
 * enumerations and literalUnion shells only.
 */
export const CATEGORY_DIRECTORIES: ReadonlyMap<string, string> = new Map([
    ['contextualValue', 'contextual_value_nodes'],
    ['count', 'count_nodes'],
    ['discriminator', 'discriminator_nodes'],
    ['display', 'display_nodes'],
    ['link', 'link_nodes'],
    ['pdaSeed', 'pda_seed_nodes'],
    ['shared', 'shared'],
    ['topLevel', ''],
    ['transform', 'transform_nodes'],
    ['type', 'type_nodes'],
    ['value', 'value_nodes'],
]);
