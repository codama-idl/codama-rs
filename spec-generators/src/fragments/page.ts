import { type Fragment, fragment, type ImportMap, importMapToString, mergeFragments } from '@codama/fragments/rust';

/**
 * Render a Rust source page from a body fragment: prepend a `use`
 * block built from the fragment's import map, then the body content.
 *
 * Imports sharing the same module prefix are collapsed into a single
 * grouped `use <module>::{A, B};` line (e.g. `use crate::{Foo, Bar};`,
 * `use codama_nodes_derive::{node_union, RegisteredNodes};`) to match
 * the hand-written convention. Single-import modules stay on their
 * own line (`use crate::Foo;`). Lines are sorted alphabetically by
 * module path.
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
    // Each line is `use <path>;`. Split each path into `<module>::<name>`,
    // grouping by module so multiple imports from the same module collapse.
    const lines = importMapToString(importMap)
        .split('\n')
        .filter(line => line !== '');

    const byModule = new Map<string, string[]>();
    const ungroupable: string[] = [];
    for (const line of lines) {
        const match = /^use (.+)::([A-Za-z0-9_]+);$/.exec(line);
        if (!match) {
            ungroupable.push(line);
            continue;
        }
        const [, mod, name] = match;
        const names = byModule.get(mod) ?? [];
        names.push(name);
        byModule.set(mod, names);
    }

    const grouped: string[] = [];
    for (const [mod, names] of byModule) {
        const sorted = [...names].toSorted((a, b) => a.localeCompare(b));
        grouped.push(sorted.length === 1 ? `use ${mod}::${sorted[0]};` : `use ${mod}::{${sorted.join(', ')}};`);
    }

    const output = [...grouped, ...ungroupable].toSorted((a, b) => a.localeCompare(b));
    return output.join('\n');
}
