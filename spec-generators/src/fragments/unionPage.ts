import { pascalCase } from '@codama/fragments';
import { type Fragment, fragment, mergeFragments } from '@codama/fragments/rust';
import type { NodeSpec, Spec, UnionSpec } from '@codama/spec';

import { INLINE_UNIONS } from '../defaults';
import { flattenNodeUnion } from '../unions';
import { getUnionHasNameImplFragment } from './hasNameImpl';
import { use } from './helpers';

/**
 * The body for one spec union's generated source file:
 *
 *   1. `#[node_union] pub enum XxxUnion { Variant(MemberNode), … }` —
 *      variants are the union's flattened leaf nodes, sorted
 *      alphabetically. Variant name = pascalCase(kind) minus the
 *      union's implied node suffix (e.g. `accountLinkNode` in
 *      `linkNode` → `Account`).
 *   2. `impl HasName for XxxUnion { … }` when every member node has a
 *      `name: stringIdentifier()` attribute.
 */
export function getUnionPageFragment(union: UnionSpec, spec: Spec): Fragment {
    const unionName = pascalCase(union.name);
    const variants = buildVariants(union, spec);

    const enumFragment = buildEnumFragment(unionName, variants);
    const hasNameFragment = getUnionHasNameImplFragment(unionName, variants);

    const blocks: (Fragment | undefined)[] = [enumFragment, hasNameFragment];
    return mergeFragments(blocks, parts => parts.join('\n\n'));
}

interface UnionVariant {
    /** The Rust variant name (PascalCase, stripped suffix). */
    readonly name: string;
    /** The wrapped node spec (for `HasName` dispatch). */
    readonly node: NodeSpec;
}

function buildVariants(union: UnionSpec, spec: Spec): readonly UnionVariant[] {
    const suffix = variantStripSuffix(union);
    return [...flattenNodeUnion(union, spec)]
        .map(node => ({ name: variantNameForNode(node.kind, suffix), node }))
        .toSorted((a, b) => a.name.localeCompare(b.name));
}

/**
 * The PascalCase suffix to strip from each leaf node's kind when
 * deriving variant names. For category-main unions it defaults to
 * `pascalCase(union.name)` (e.g. `LinkNode`, `CountNode`). For inline
 * unions, the suffix is taken from {@link INLINE_UNIONS}; inline
 * unions whose members don't share a common suffix can omit
 * `stripSuffix` (no stripping happens then).
 */
function variantStripSuffix(union: UnionSpec): string {
    const inline = INLINE_UNIONS.get(union.name);
    if (inline !== undefined) return inline.stripSuffix ?? '';
    return pascalCase(union.name);
}

/**
 * Variant name = pascalCase(kind) minus the implied suffix.
 *
 *   - `accountLinkNode`      in union `linkNode`             → `Account`
 *   - `fixedCountNode`       in union `countNode`            → `Fixed`
 *   - `numberValueNode`      in inline `constantPdaSeedValue`
 *     (stripSuffix = `ValueNode`)                            → `Number`
 *   - `programIdValueNode`   in inline `constantPdaSeedValue`
 *     (stripSuffix = `ValueNode`)                            → `ProgramId`
 */
function variantNameForNode(nodeKind: string, suffix: string): string {
    const pascal = pascalCase(nodeKind);
    if (suffix === '') return pascal;
    return pascal.endsWith(suffix) ? pascal.slice(0, pascal.length - suffix.length) : pascal;
}

function buildEnumFragment(unionName: string, variants: readonly UnionVariant[]): Fragment {
    const lines = mergeFragments(
        variants.map(v => fragment`${v.name}(${use(`crate::${pascalCase(v.node.kind)}`)}),`),
        parts => parts.join('\n'),
    );
    return fragment`#[${use('codama_nodes_derive::node_union')}]\npub enum ${unionName} {\n${lines}\n}`;
}
