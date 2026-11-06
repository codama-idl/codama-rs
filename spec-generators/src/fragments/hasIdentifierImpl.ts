import { pascalCase } from '@codama/fragments';
import { type Fragment, fragment, mergeFragments } from '@codama/fragments/rust';
import type { AttributeSpec, NodeSpec } from '@codama/spec';

import { use } from './helpers';

/**
 * `true` for a required `identifier: stringIdentifier()` attribute, which renders as `identifier: CamelCaseString`.
 * An optional one renders as `Option<CamelCaseString>`, which `HasIdentifier::identifier()` cannot return.
 */
export function isIdentifierAttribute(attr: AttributeSpec): boolean {
    return (
        attr.name === 'identifier' &&
        attr.type.kind === 'string' &&
        attr.type.constraint === 'identifier' &&
        attr.optional !== true
    );
}

/** `true` when the node has a required identifier, the only condition for a `HasIdentifier` impl. */
export function nodeHasIdentifier(node: NodeSpec): boolean {
    return node.attributes.some(isIdentifierAttribute);
}

/**
 * Render the `impl HasIdentifier for XxxNode` block for a struct, or
 * `undefined` when the node doesn't satisfy {@link nodeHasIdentifier}.
 *
 * The struct's own name is interpolated (not `use`d) because the
 * struct lives in the same file; importing it would emit an invalid
 * `use crate::Self;` line.
 */
export function getStructHasIdentifierImplFragment(node: NodeSpec): Fragment | undefined {
    if (!nodeHasIdentifier(node)) return undefined;
    const structName = pascalCase(node.kind);
    return fragment`impl ${use('crate::HasIdentifier')} for ${structName} {\nfn identifier(&self) -> &${use(
        'crate::CamelCaseString',
    )} {\n&self.identifier\n}\n}`;
}

/**
 * Render the `impl HasIdentifier for XxxUnion` block for a union enum
 * whose every member node satisfies {@link nodeHasIdentifier},
 * dispatching each variant to the underlying node's `identifier()` via a
 * `match` arm. Returns `undefined` when any member lacks an identifier.
 */
export function getUnionHasIdentifierImplFragment(
    unionName: string,
    variants: readonly { readonly name: string; readonly node: NodeSpec }[],
): Fragment | undefined {
    if (!variants.every(v => nodeHasIdentifier(v.node))) return undefined;
    const arms = mergeFragments(
        variants.map(v => fragment`${unionName}::${v.name}(node) => node.identifier(),`),
        parts => parts.join('\n'),
    );
    return fragment`impl ${use('crate::HasIdentifier')} for ${unionName} {\nfn identifier(&self) -> &${use(
        'crate::CamelCaseString',
    )} {\nmatch self {\n${arms}\n}\n}\n}`;
}
