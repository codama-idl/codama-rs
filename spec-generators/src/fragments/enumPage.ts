import { pascalCase } from '@codama/fragments';
import { type Fragment, fragment, mergeFragments } from '@codama/fragments/rust';
import type { EnumerationSpec } from '@codama/spec';

import { use } from './helpers';

/**
 * Render one spec enumeration as a Rust `enum` shell:
 *
 * ```rust
 * #[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
 * #[serde(rename_all = "camelCase")]
 * pub enum Foo {
 *     /// Variant docs from the spec, if any.
 *     Variant,
 *     // …
 * }
 * ```
 *
 * Variant names are `pascalCase(variant.name)` and variants are
 * emitted in the order declared by the spec. Bespoke per-enum impls
 * (`Default`, `TryFrom`, custom `Serialize`/`Deserialize`, …) stay
 * hand-written in the non-generated companion file alongside the
 * generated shell — same split as e.g. `Number` (generated reference,
 * hand-written impls).
 *
 * The generator deliberately does NOT emit `#[derive(Default)]`: the
 * spec doesn't carry a "default variant" notion, so any required
 * `Default` impl is hand-written.
 *
 * Today every spec variant name PascalCases to a valid Rust identifier
 * AND matches the Rust convention (e.g. `programId` → `ProgramId`,
 * `u8` → `U8`). The one historical outlier (`Endianness::{Big, Little}`
 * for spec variants `[be, le]`) is reconciled by renaming Rust to
 * `{Be, Le}` rather than carrying a per-enum override.
 */
export function getEnumPageFragment(enumeration: EnumerationSpec): Fragment {
    const enumName = pascalCase(enumeration.name);
    const variantLines = mergeFragments(
        enumeration.variants.map(v => fragment`${formatDocs(v.docs)}${pascalCase(v.name)},`),
        parts => parts.join('\n'),
    );

    const docs = formatDocs(enumeration.docs);
    const derives = fragment`#[derive(Debug, PartialEq, Eq, Clone, Copy, ${use('serde::Serialize')}, ${use('serde::Deserialize')})]`;

    return fragment`${docs}${derives}\n#[serde(rename_all = "camelCase")]\npub enum ${enumName} {\n${variantLines}\n}\n`;
}

function formatDocs(docs: readonly string[] | undefined): string {
    if (!docs || docs.length === 0) return '';
    return docs.map(d => `/// ${d}`).join('\n') + '\n';
}
