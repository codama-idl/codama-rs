/**
 * Default options for the v1 spec. Future spec versions can ship
 * their own defaults alongside these without breaking v1 callers.
 */

/**
 * How a category's nodes reach the top-level `crate::Node` enum.
 *
 *   - `wrapped`: generator emits
 *     `impl From<XxxNode> for crate::Node` routing through the
 *     category union (`crate::Node::<variant>(val.into())`).
 *   - `direct`: the node is a direct `Node` variant and `Node`'s
 *     `#[derive(From)]` auto-generates the impl, so the generator
 *     emits no `From` of its own.
 */
export type CategoryRouting = { readonly mode: 'wrapped'; readonly nodeVariant: string } | { readonly mode: 'direct' };

/**
 * Routing table for spec categories whose nodes the generator emits in
 * v1. Categories absent from this map are not generated yet.
 */
export const CATEGORY_ROUTING: ReadonlyMap<string, CategoryRouting> = new Map([
    ['contextualValue', { mode: 'wrapped', nodeVariant: 'ContextualValue' }],
    ['count', { mode: 'wrapped', nodeVariant: 'Count' }],
    ['discriminator', { mode: 'wrapped', nodeVariant: 'Discriminator' }],
    ['link', { mode: 'wrapped', nodeVariant: 'Link' }],
    ['pdaSeed', { mode: 'wrapped', nodeVariant: 'PdaSeed' }],
    ['topLevel', { mode: 'direct' }],
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
    ['link', 'link_nodes'],
    ['pdaSeed', 'pda_seed_nodes'],
    ['shared', 'shared'],
    ['topLevel', ''],
    ['type', 'type_nodes'],
    ['value', 'value_nodes'],
]);

/**
 * Per-field Rust-type overrides for cases where the spec's TypeExpr
 * maps to a bespoke Rust type that can't be expressed mechanically.
 * Keyed by `"<nodeKind>.<attrName>"`.
 *
 *   - `numberValueNode.number`: spec says `float(f64)` but the Rust
 *     crate uses a bespoke `Number` enum
 *     (`UnsignedInteger(u64) | SignedInteger(i64) | Float(f64)`) with
 *     a custom `serde(from/into = "JsonNumber")` and 8 `From<uN/iN/fN>`
 *     impls. The struct + derives are still generated; the field type
 *     resolves to `crate::Number` via this override and the `Number`
 *     enum + its impls stay hand-written in
 *     `value_nodes/number_value_node.rs`.
 */
export const FIELD_TYPE_OVERRIDES: ReadonlyMap<string, string> = new Map([['numberValueNode.number', 'crate::Number']]);
