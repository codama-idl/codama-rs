import { pascalCase } from '@codama/fragments';
import { addFragmentImports, type Fragment, fragment, mergeFragments } from '@codama/fragments/rust';
import type { Spec, UnionSpec } from '@codama/spec';

import { flattenNodeUnion, getRegisteredOnlyLeafKinds, getUnionCategory, getUnionVariantName } from '../unions';
import { use } from './helpers';

/**
 * The body for a category union that has `#[registered]`-only
 * variants (e.g. `value`'s `RegisteredValueNode`). Emits:
 *
 *   `#[derive(RegisteredNodes)] #[node_union] #[derive(NodeConversions)] pub enum Registered<X> {
 *       <standalone variants, alphabetical>
 *
 *       #[registered]
 *       <extra variant>
 *       …
 *   }`
 *
 * The `RegisteredNodes` derive macro produces the standalone twin
 * (`X`) plus the `From`/`TryFrom` bridges between them, so the
 * generated file replaces the entire hand-written `value_node.rs`
 * pattern.
 *
 * The twin is a copy of the attributes that follow `#[derive(RegisteredNodes)]`,
 * so `#[node_union]` and `#[derive(NodeConversions)]` must come after it for
 * the twin to get them too. Keeping `NodeConversions` in its own `derive` after
 * `#[node_union]` also stops `rustfmt` from merging it into the first one.
 *
 * Standalone variants are sorted alphabetically by their stripped
 * variant name (consistent with `unionPage.ts`); `#[registered]`
 * variants follow in spec declaration order (the `registered<X>`
 * union's member order). The blank line between the two sections is
 * preserved by `rustfmt`.
 */
export function getRegisteredUnionPageFragment(union: UnionSpec, spec: Spec): Fragment {
    const enumName = `Registered${pascalCase(union.name)}`;
    // The main union of a category implies its category word, see `getUnionVariantName`.
    const category = getUnionCategory(union, spec);
    const standaloneVariants = [...flattenNodeUnion(union, spec)]
        .map(node => ({ kind: node.kind, name: getUnionVariantName(node.kind, category) }))
        .toSorted((a, b) => a.name.localeCompare(b.name));
    const registeredOnlyVariants = getRegisteredOnlyLeafKinds(union, spec).map(kind => ({
        kind,
        name: getUnionVariantName(kind, category),
    }));

    const standaloneLines = mergeFragments(
        standaloneVariants.map(v => fragment`${v.name}(${use(`crate::${pascalCase(v.kind)}`)}),`),
        parts => parts.join('\n'),
    );
    const registeredLines = mergeFragments(
        registeredOnlyVariants.map(v => fragment`#[registered]\n${v.name}(${use(`crate::${pascalCase(v.kind)}`)}),`),
        parts => parts.join('\n'),
    );

    // Two sections separated by a blank line so rustfmt preserves the
    // visual split between standalone and `#[registered]` variants.
    const body = mergeFragments([standaloneLines, registeredLines], parts => parts.join('\n\n'));

    // The `RegisteredNodes` derive macro expansion calls `.kind()` on
    // each variant, which requires `HasKind` to be in scope.
    return addFragmentImports(
        fragment`#[derive(${use('codama_nodes_derive::RegisteredNodes')})]\n#[${use('codama_nodes_derive::node_union')}]\n#[derive(${use('codama_nodes_derive::NodeConversions')})]\n#[non_exhaustive]\npub enum ${enumName} {\n${body}\n}`,
        ['crate::HasKind'],
    );
}
