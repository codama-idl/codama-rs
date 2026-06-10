/**
 * Default options for the v1 spec. Future spec versions can ship
 * their own defaults alongside these without breaking v1 callers.
 */

/**
 * Per-spec-category routing: tells the generator which
 * `crate::Node::<variant>` to wrap a node in for its
 * `From<Self> for crate::Node` impl. For categories reached through a
 * union enum (e.g. `link` → `LinkNode`), the impl does
 * `crate::Node::Link(val.into())` — `.into()` lands in the category
 * union first, then the variant wraps it in `Node`. `topLevel` skips
 * the union step; that distinction lands when `topLevel` does.
 */
export interface CategoryRouting {
    /** The `crate::Node::<variant>` constructor the node routes through. */
    readonly nodeVariant: string;
}

/**
 * Routing table for spec categories whose nodes the generator emits in
 * v1. Categories absent from this map are not generated yet (today
 * that's every category except `link`).
 */
export const CATEGORY_ROUTING: ReadonlyMap<string, CategoryRouting> = new Map([
    ['count', { nodeVariant: 'Count' }],
    ['discriminator', { nodeVariant: 'Discriminator' }],
    ['link', { nodeVariant: 'Link' }],
    ['pdaSeed', { nodeVariant: 'PdaSeed' }],
    ['value', { nodeVariant: 'Value' }],
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
 * Per-inline-union configuration.
 *
 * Most emittable unions are the category-main union (the standalone
 * twin of a `registered…`); those have an obvious variant-naming
 * rule (strip the category suffix). Inline / synthetic unions —
 * unions used in a single attribute, with no registered twin — need
 * an explicit allowlist so the generator knows to emit them, plus
 * their own variant-naming convention.
 *
 * Each entry's key is the spec union name; the value carries:
 *
 *   - `stripSuffix`: the PascalCase suffix to strip from each leaf
 *     node's kind when deriving its Rust variant name. For example,
 *     `constantPdaSeedValue`'s flattened members include
 *     `numberValueNode`, `programIdValueNode`, …; stripping the
 *     `ValueNode` suffix yields variant names `Number`, `ProgramId`,
 *     etc. — matching the convention used by today's hand-written
 *     inline unions in the same category family.
 */
export interface InlineUnionConfig {
    readonly stripSuffix?: string;
}

export const INLINE_UNIONS: ReadonlyMap<string, InlineUnionConfig> = new Map([
    ['constantPdaSeedValue', { stripSuffix: 'ValueNode' }],
    ['enumValuePayload', { stripSuffix: 'ValueNode' }],
]);

/**
 * Spec union names the generator must NOT emit because their Rust
 * counterpart is bespoke hand-written code that can't be reproduced
 * mechanically.
 *
 *   - `valueNode` is the category dispatch union for the `value`
 *     category, but in Rust it's the auto-derived standalone twin of
 *     `RegisteredValueNode` (`#[derive(RegisteredNodes)]`), which has
 *     the `registered`/`standalone` split with `#[registered]`
 *     variants. The generator doesn't model that yet; the
 *     hand-written `value_nodes/value_node.rs` stays canonical.
 *
 * Per-node structs in these categories are still generated normally;
 * only the category union itself is skipped.
 */
export const HAND_WRITTEN_UNIONS: ReadonlySet<string> = new Set(['valueNode']);

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
