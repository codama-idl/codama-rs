import { pascalCase } from '@codama/fragments';
import { type Fragment, fragment, mergeFragments } from '@codama/fragments/rust';
import type { NodeSpec, Spec, UnionSpec } from '@codama/spec';

import { flattenNodeUnion, getUnionCategory, getUnionVariantName } from '../unions';
import { getUnionHasIdentifierImplFragment } from './hasIdentifierImpl';
import { use } from './helpers';

/**
 * The body for one spec union's generated source file:
 *
 *   1. `#[derive(NodeConversions)] #[node_union] pub enum XxxUnion { Variant(MemberNode), … }` —
 *      variants are the union's flattened leaf nodes, sorted
 *      alphabetically. Variant name = pascalCase(kind) minus the
 *      union's implied node suffix (e.g. `accountLinkNode` in
 *      `linkNode` → `Account`).
 *   2. `impl HasIdentifier for XxxUnion { … }` when every member node has a
 *      required identifier.
 */
export function getUnionPageFragment(union: UnionSpec, spec: Spec): Fragment {
    const unionName = pascalCase(union.name);
    const variants = getUnionVariants(union, spec);

    const enumFragment = buildEnumFragment(unionName, variants);
    const hasIdentifierFragment = getUnionHasIdentifierImplFragment(unionName, variants);

    const blocks: (Fragment | undefined)[] = [enumFragment, hasIdentifierFragment];
    return mergeFragments(blocks, parts => parts.join('\n\n'));
}

interface UnionVariant {
    /** The Rust variant name, see {@link getUnionVariantName}. */
    readonly name: string;
    /** The wrapped node spec (for `HasIdentifier` dispatch). */
    readonly node: NodeSpec;
}

/**
 * The leaves of the union with their variant names, sorted alphabetically. The main union of a
 * category implies its category word (`LinkNode::Account`); an inline union keeps the whole name
 * (`PdaSeedValueValue::IntegerValue`). See {@link getUnionVariantName}.
 */
function getUnionVariants(union: UnionSpec, spec: Spec): readonly UnionVariant[] {
    const category = getUnionCategory(union, spec);
    return [...flattenNodeUnion(union, spec)]
        .map(node => ({ name: getUnionVariantName(node.kind, category), node }))
        .toSorted((a, b) => a.name.localeCompare(b.name));
}

function buildEnumFragment(unionName: string, variants: readonly UnionVariant[]): Fragment {
    const lines = mergeFragments(
        variants.map(v => fragment`${v.name}(${use(`crate::${pascalCase(v.node.kind)}`)}),`),
        parts => parts.join('\n'),
    );
    return fragment`#[derive(${use('codama_nodes_derive::NodeConversions')})]\n#[${use('codama_nodes_derive::node_union')}]\n#[non_exhaustive]\npub enum ${unionName} {\n${lines}\n}`;
}
