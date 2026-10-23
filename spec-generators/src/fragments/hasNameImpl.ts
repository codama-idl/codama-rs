import { pascalCase } from '@codama/fragments';
import { type Fragment, fragment, mergeFragments } from '@codama/fragments/rust';
import type { AttributeSpec, NodeSpec } from '@codama/spec';

import { TEMP_NAME_NODE_EXCEPTIONS } from '../defaults';
import { use } from './helpers';

/**
 * `true` when the attribute renders as the Rust field `identifier` with `#[serde(rename = "name")]`.
 * Applies to a `name: stringIdentifier()` attribute unless the node is in {@link TEMP_NAME_NODE_EXCEPTIONS}.
 * TODO(spec v2): keyed on the v1 `name` attribute and the temporary exception list. At the v2 pin, key on `identifier`.
 */
export function isIdentifierAttribute(node: NodeSpec, attr: AttributeSpec): boolean {
    return (
        attr.name === 'name' &&
        attr.type.kind === 'string' &&
        attr.type.constraint === 'identifier' &&
        !TEMP_NAME_NODE_EXCEPTIONS.has(node.kind)
    );
}

/**
 * `true` when the node has a Rust field `identifier`, the only condition for a `HasIdentifier` impl.
 * If the spec renames the `name` attribute, nothing matches and the generator skips the impl without an error.
 */
export function nodeHasIdentifier(node: NodeSpec): boolean {
    return node.attributes.some(a => isIdentifierAttribute(node, a));
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
