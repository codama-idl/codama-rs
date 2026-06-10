import { type Fragment, fragment, type ImportMap, importMapToString, mergeFragments } from '@codama/fragments/rust';

/**
 * Render a Rust source page from a body fragment: prepend a `use`
 * block built from the fragment's import map, then the body content.
 *
 * `use crate::Foo;` lines are collapsed into a single
 * `use crate::{Foo, Bar};` to match the hand-written convention;
 * non-`crate::` paths stay on their own lines, sorted alphabetically.
 *
 * This grouping has to happen here because stable `rustfmt` (the
 * codama-rs toolchain) sorts `use` lines but won't merge them —
 * `imports_granularity` is a nightly-only knob.
 */
export function getPageFragment(body: Fragment): Fragment {
    if (body.imports.size === 0) return body;
    const importBlock = formatImports(body.imports);
    return mergeFragments([fragment`${importBlock}`, body], parts => parts.join('\n\n').trimEnd() + '\n');
}

function formatImports(importMap: ImportMap): string {
    const lines = importMapToString(importMap)
        .split('\n')
        .filter(line => line !== '');

    const crateRefs: string[] = [];
    const other: string[] = [];
    for (const line of lines) {
        const match = /^use crate::([A-Za-z0-9_]+);$/.exec(line);
        if (match) crateRefs.push(match[1]);
        else other.push(line);
    }

    const output: string[] = [];
    if (crateRefs.length > 0) {
        const sorted = [...crateRefs].toSorted((a, b) => a.localeCompare(b));
        output.push(sorted.length === 1 ? `use crate::${sorted[0]};` : `use crate::{${sorted.join(', ')}};`);
    }
    output.push(...other.toSorted((a, b) => a.localeCompare(b)));
    return output.join('\n');
}
