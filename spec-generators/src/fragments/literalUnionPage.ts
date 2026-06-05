import { type Fragment, fragment, mergeFragments } from '@codama/fragments/rust';

import { literalUnionVariantName, type LiteralUnionTypeRef } from '../literalUnions';

/**
 * Render one `literalUnion` as a Rust `enum` **shell**:
 *
 * ```rust
 * #[derive(Debug, PartialEq, Eq, Clone, Copy)]
 * pub enum IsSigner {
 *     True,
 *     False,
 *     Either,
 * }
 * ```
 *
 * Note the derive set: **`Debug, PartialEq, Eq, Clone, Copy` only** —
 * NO `Serialize`/`Deserialize` and NO `Default`, deliberately. A
 * `literalUnion`'s wire format is heterogeneous (e.g. JSON `true`,
 * `false`, or `"either"`) and cannot be expressed via `#[serde(rename)]`
 * or `rename_all`; it requires a hand-written `Serialize`/`Deserialize`
 * pair using a `Visitor` that accepts multiple JSON types. `Default`
 * is likewise hand-written because the spec doesn't carry a
 * default-variant notion. Both impls live in the hand-written
 * companion file alongside the generated shell — the same split
 * already used by `Number` and the enumeration `Default`s from PR #9.
 *
 * Variants are emitted in spec declaration order using
 * {@link literalUnionVariantName} (`true`→`True`, `false`→`False`,
 * string→PascalCase).
 *
 * No `use` imports are required: the standard derives are all in the
 * Rust prelude.
 */
export function getLiteralUnionPageFragment(ref: LiteralUnionTypeRef): Fragment {
    const variantLines = mergeFragments(
        ref.values.map(v => fragment`${literalUnionVariantName(v)},`),
        parts => parts.join('\n'),
    );
    return fragment`#[derive(Debug, PartialEq, Eq, Clone, Copy)]\npub enum ${ref.typeName} {\n${variantLines}\n}\n`;
}
