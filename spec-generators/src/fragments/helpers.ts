import { addFragmentImports, type Fragment, fragment } from '@codama/fragments/rust';

/**
 * Build a fragment that references a Rust identifier reachable via
 * the given fully-qualified path. The fragment's content is the
 * trailing segment of the path; its import map carries a single
 * matching `use <path>;` entry.
 *
 * Rust analogue of the JS subpath's `use(identifier, module)` helper.
 * Unlike the JS version, `use` takes the full path in one string
 * because Rust paths are absolute and self-describing.
 *
 * @example
 * ```ts
 * use('crate::CamelCaseString');
 * // content: `CamelCaseString`
 * // imports: { 'crate::CamelCaseString' }
 *
 * use('codama_nodes_derive::node');
 * // content: `node`
 * // imports: { 'codama_nodes_derive::node' }
 * ```
 */
export function use(path: string): Fragment {
    const name = path.split('::').at(-1);
    if (!name) throw new Error(`use(): empty Rust path "${path}"`);
    return addFragmentImports(fragment`${name}`, [path]);
}
