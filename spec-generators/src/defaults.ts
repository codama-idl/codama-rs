/**
 * Default options for the v1 spec. Future spec versions can ship
 * their own defaults alongside these without breaking v1 callers.
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
 * Routing table for spec categories whose nodes the generator emits in
 * v1. Categories absent from this map are not generated yet.
 */
export const CATEGORY_ROUTING: ReadonlyMap<string, CategoryRouting> = new Map([
    ['contextualValue', { mode: 'wrapped', nodeVariant: 'ContextualValue' }],
    ['count', { mode: 'wrapped', nodeVariant: 'Count' }],
    ['discriminator', { mode: 'wrapped', nodeVariant: 'Discriminator' }],
    ['display', { mode: 'wrapped', nodeVariant: 'Display' }],
    ['link', { mode: 'wrapped', nodeVariant: 'Link' }],
    ['pdaSeed', { mode: 'wrapped', nodeVariant: 'PdaSeed' }],
    ['topLevel', { mode: 'direct' }],
    ['type', { mode: 'wrapped', nodeVariant: 'Type', emitUnions: false }],
    ['value', { mode: 'wrapped', nodeVariant: 'Value' }],
]);

/**
 * Mapping from spec category name to the output subdirectory the
 * generator emits its entities into (relative to `generated/`). The
 * empty string places `topLevel` entities at the root.
 *
 * Only categories with a corresponding routing entry in
 * {@link CATEGORY_ROUTING} are emitted in v1; the rest stay
 * hand-written for now and will land in future PRs.
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
    ['type', 'type_nodes'],
    ['value', 'value_nodes'],
]);

/**
 * v1 nodes whose `name` attribute keeps the Rust field `name`.
 * Every other `name: stringIdentifier()` attribute renders as the Rust field `identifier`, the spec v2 name.
 * Spec v2 has no `identifier` on these nodes.
 * TODO(spec v2): temporary list, not defined by the spec. Delete it at the spec v2 pin.
 */
export const TEMP_NAME_NODE_EXCEPTIONS: ReadonlySet<string> = new Set([
    // Spec v2 renames the attribute to `namespace` or `path`, or the node to `dataValueNode`.
    'argumentValueNode',
    'fieldDiscriminatorNode',
    'pluginNode',
    // Spec v2 removes the node or merges it into `enumVariantTypeNode`.
    // The `enumVariantTypeNode` merge replaces the 3 enum variant structs, so a field rename here gains nothing.
    'enumEmptyVariantTypeNode',
    'enumStructVariantTypeNode',
    'enumTupleVariantTypeNode',
    'instructionArgumentLinkNode',
    'instructionArgumentNode',
    'resolverValueNode',
]);
