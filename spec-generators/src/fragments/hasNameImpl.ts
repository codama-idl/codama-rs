import { pascalCase } from '@codama/fragments';
import { type Fragment, fragment, mergeFragments } from '@codama/fragments/rust';
import type { NodeSpec } from '@codama/spec';

import { use } from './helpers';

/**
 * `true` when the node has a `name: stringIdentifier()` attribute —
 * the only condition under which the generator emits a `HasName`
 * impl. Matches the spec attribute shape (not the field type) so a
 * future spec rename surfaces as a build failure rather than a silent
 * skip.
 */
export function nodeHasName(node: NodeSpec): boolean {
    return node.attributes.some(
        a => a.name === 'name' && a.type.kind === 'string' && a.type.constraint === 'identifier',
    );
}

/**
 * Render the `impl HasName for XxxNode` block for a struct, or
 * `undefined` when the node doesn't satisfy {@link nodeHasName}.
 *
 * The struct's own name is interpolated (not `use`d) because the
 * struct lives in the same file; importing it would emit an invalid
 * `use crate::Self;` line.
 */
export function getStructHasNameImplFragment(node: NodeSpec): Fragment | undefined {
    if (!nodeHasName(node)) return undefined;
    const structName = pascalCase(node.kind);
    return fragment`impl ${use('crate::HasName')} for ${structName} {\nfn name(&self) -> &${use(
        'crate::CamelCaseString',
    )} {\n&self.name\n}\n}`;
}

/**
 * Render the `impl HasName for XxxUnion` block for a union enum
 * whose every member node has a `name: stringIdentifier()` attribute,
 * dispatching each variant to the underlying node's `name()` via a
 * `match` arm. Returns `undefined` when any member lacks a name.
 */
export function getUnionHasNameImplFragment(
    unionName: string,
    variants: readonly { readonly name: string; readonly node: NodeSpec }[],
): Fragment | undefined {
    if (!variants.every(v => nodeHasName(v.node))) return undefined;
    const arms = mergeFragments(
        variants.map(v => fragment`${unionName}::${v.name}(node) => node.name(),`),
        parts => parts.join('\n'),
    );
    return fragment`impl ${use('crate::HasName')} for ${unionName} {\nfn name(&self) -> &${use(
        'crate::CamelCaseString',
    )} {\nmatch self {\n${arms}\n}\n}\n}`;
}
