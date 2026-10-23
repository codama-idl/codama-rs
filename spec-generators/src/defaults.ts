/**
 * Default options for the spec v2 generator.
 */

/**
 * How a category's nodes reach the top-level `crate::Node` enum and
 * whether its unions are emitted by the generator.
 *
 *   - `wrapped`: generator emits `impl From<XxxNode> for crate::Node`
 *     routing through the category union (`crate::Node::<variant>(val.into())`).
 *   - `direct`: the node is a direct `Node` variant and `Node`'s
 *     `#[derive(From)]` auto-generates the impl.
 *
 * `emitUnions` defaults to `true`; set to `false` for categories whose
 * unions are too bespoke for the standard renderers (currently `type`).
 */
export type CategoryRouting = (
    | { readonly mode: 'wrapped'; readonly nodeVariant: string }
    | { readonly mode: 'direct' }
) & { readonly emitUnions?: boolean };

/**
 * Routing table for spec categories whose nodes the generator emits.
 * Categories absent from this map are not generated.
 */
export const CATEGORY_ROUTING: ReadonlyMap<string, CategoryRouting> = new Map([
    ['contextualValue', { mode: 'wrapped', nodeVariant: 'ContextualValue' }],
    ['count', { mode: 'wrapped', nodeVariant: 'Count' }],
    ['discriminator', { mode: 'wrapped', nodeVariant: 'Discriminator' }],
    ['display', { mode: 'wrapped', nodeVariant: 'Display' }],
    ['link', { mode: 'wrapped', nodeVariant: 'Link' }],
    ['pdaSeed', { mode: 'wrapped', nodeVariant: 'PdaSeed' }],
    ['topLevel', { mode: 'direct' }],
    ['transform', { mode: 'wrapped', nodeVariant: 'Transform' }],
    ['type', { mode: 'wrapped', nodeVariant: 'Type', emitUnions: false }],
    ['value', { mode: 'wrapped', nodeVariant: 'Value' }],
]);

/**
 * Mapping from spec category name to the output subdirectory the
 * generator emits its entities into (relative to `generated/`). The
 * empty string places `topLevel` entities at the root.
 *
 * Only categories with a corresponding routing entry in
 * {@link CATEGORY_ROUTING} get node and union pages. `shared` holds
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
