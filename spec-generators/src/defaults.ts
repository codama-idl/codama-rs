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
 * Per-spec-union Rust-side name overrides.
 *
 * Most spec union names map directly to their PascalCase Rust enum:
 * `linkNode` → `LinkNode`, `typeNode` → `TypeNode`. A handful of
 * unions are exposed under a different name in the Rust crate for
 * historic API-stability reasons; those overrides live here.
 *
 * Keys are spec union names (camelCase); values are Rust enum names
 * (PascalCase). Unions absent from this map use the default
 * `pascalCase(specName)` mapping.
 */
export const UNION_NAME_OVERRIDES: ReadonlyMap<string, string> = new Map([
    ['conditionalValueCondition', 'ConditionNode'],
    ['instructionByteDeltaValue', 'InstructionByteDeltaNodeValue'],
    ['instructionRemainingAccountsValue', 'InstructionRemainingAccountsNodeValue'],
    ['pdaSeedValueValue', 'PdaSeedValueValueNode'],
    ['pdaValuePda', 'PdaValue'],
    ['pdaValueProgramId', 'PdaProgramIdValueNode'],
]);

/**
 * Per-spec-enumeration Rust-side name overrides. Same rule as
 * {@link UNION_NAME_OVERRIDES}: present only when the Rust enum name
 * differs from `pascalCase(specName)`.
 */
export const ENUMERATION_NAME_OVERRIDES: ReadonlyMap<string, string> = new Map([
    ['endianness', 'Endian'],
    ['optionalAccountStrategy', 'InstructionOptionalAccountStrategy'],
]);
