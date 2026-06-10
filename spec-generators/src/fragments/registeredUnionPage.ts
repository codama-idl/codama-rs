import { pascalCase } from '@codama/fragments';
import { addFragmentImports, type Fragment, fragment, mergeFragments } from '@codama/fragments/rust';
import type { Spec, UnionSpec } from '@codama/spec';

import { flattenNodeUnion, getRegisteredOnlyLeafKinds, getRegisteredUnionStripSuffix } from '../unions';
import { use } from './helpers';

/**
 * The body for a category union that has `#[registered]`-only
 * variants (e.g. `value`'s `RegisteredValueNode`). Emits:
 *
 *   `#[derive(RegisteredNodes)] #[node_union] pub enum Registered<X> {
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
 * Standalone variants are sorted alphabetically by their stripped
 * variant name (consistent with `unionPage.ts`); `#[registered]`
 * variants follow in spec declaration order (the `registered<X>`
 * union's member order). The blank line between the two sections is
 * preserved by `rustfmt`.
 */
export function getRegisteredUnionPageFragment(union: UnionSpec, spec: Spec): Fragment {
    const enumName = `Registered${pascalCase(union.name)}`;
    // Strip the longest common PascalCase suffix across BOTH standalone
    // and registered-only leaves. `valueNode` → `'ValueNode'` (leaves
    // end in `ValueNode`); `contextualValueNode` → also `'ValueNode'`
    // (leaves like `accountValueNode` end in `ValueNode`, NOT
    // `ContextualValueNode`).
    const suffix = getRegisteredUnionStripSuffix(union, spec);

    const registeredOnlyKinds = new Set(getRegisteredOnlyLeafKinds(union, spec));
    const standaloneLeaves = [...flattenNodeUnion(union, spec)];
    const standaloneVariants = standaloneLeaves
        .map(node => ({ kind: node.kind, name: variantNameForKind(node.kind, suffix) }))
        .toSorted((a, b) => a.name.localeCompare(b.name));
    const registeredOnlyVariants = getRegisteredOnlyLeafKinds(union, spec).map(kind => ({
        kind,
        name: variantNameForKind(kind, suffix),
    }));

    // Sanity check: the registered-only kinds must not overlap with the standalone set.
    for (const v of registeredOnlyVariants) {
        if (!registeredOnlyKinds.has(v.kind)) {
            throw new Error(`unexpected variant kind "${v.kind}" while building ${enumName}`);
        }
    }

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
        fragment`#[derive(${use('codama_nodes_derive::RegisteredNodes')})]\n#[${use('codama_nodes_derive::node_union')}]\npub enum ${enumName} {\n${body}\n}`,
        ['crate::HasKind'],
    );
}

function variantNameForKind(kind: string, suffix: string): string {
    const pascal = pascalCase(kind);
    return pascal.endsWith(suffix) ? pascal.slice(0, pascal.length - suffix.length) : pascal;
}
