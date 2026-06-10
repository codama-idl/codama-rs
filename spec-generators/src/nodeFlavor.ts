import type { NodeSpec, Spec } from '@codama/spec';

/**
 * Which `codama_nodes_derive` macro a generated node struct uses.
 *
 *   - `node`: the default. Used by every non-`type`-category node and
 *     by `type`-category nodes that aren't themselves type nodes (the
 *     three `enumXxxVariantTypeNode`s + `structFieldTypeNode`).
 *   - `type_node`: a `type`-category node that participates in the
 *     `TypeNode` union (the 17 non-nestable, non-enum-variant ones).
 *     Expands to `#[node] + #[derive(TypeNode)]`.
 *   - `nestable_type_node`: a `type`-category node that is a wrapper
 *     in `nestedTypeNode.wrappers`. NOT emitted by the generator in
 *     v1 — these stay hand-written; see {@link isNestableNode}.
 */
export type NodeMacroFlavor = 'node' | 'type_node' | 'nestable_type_node';

/**
 * `true` when the node is a wrapper in the `nestedTypeNode` nested
 * union (i.e. it's a nestable type node). These nodes are generic
 * over `<T: TypeNodeUnionTrait>` in Rust and carry bespoke
 * nest/un-nest conversions plus `NestedTypeNodeTrait` impls — the
 * generator skips them and they remain hand-written.
 */
export function isNestableNode(nodeKind: string, spec: Spec): boolean {
    return getNestableNodeKinds(spec).has(nodeKind);
}

/**
 * The set of node kinds that are wrappers in `nestedTypeNode`. Pulled
 * from the spec's `type` category nested unions; today this is
 * exactly seven nodes (`fixedSize`, `sizePrefix`, `pre/postOffset`,
 * `sentinel`, `hiddenPrefix`, `hiddenSuffix`). Cached on the spec
 * object would be nice but the function is cheap enough.
 */
export function getNestableNodeKinds(spec: Spec): ReadonlySet<string> {
    const out = new Set<string>();
    for (const cat of spec.categories) {
        for (const nu of cat.nestedUnions) {
            for (const w of nu.wrappers) out.add(w);
        }
    }
    return out;
}

/**
 * Determine which derive macro to use for a generated node struct.
 *
 * Rules (spec-derived):
 *
 *   - `type`-category node ∈ `nestedTypeNode.wrappers` → `nestable_type_node`
 *     (but skipped from emission — kept here so callers can detect it).
 *   - `type`-category node ∈ `enumVariantTypeNode` members, or the node
 *     is `structFieldTypeNode` → `node` (these aren't type nodes per se;
 *     they live alongside the type unions but route through
 *     `Node::Type(...)` like a regular `#[node]`).
 *   - any other `type`-category node → `type_node`.
 *   - any other category → `node`.
 */
export function getNodeMacroFlavor(node: NodeSpec, spec: Spec): NodeMacroFlavor {
    const typeCat = spec.categories.find(c => c.name === 'type');
    if (!typeCat) return 'node';
    const typeKinds = new Set(typeCat.nodes.map(n => n.kind));
    if (!typeKinds.has(node.kind)) return 'node';

    if (isNestableNode(node.kind, spec)) return 'nestable_type_node';

    const enumVariantUnion = typeCat.unions.find(u => u.name === 'enumVariantTypeNode');
    const enumVariantMembers = new Set(
        (enumVariantUnion?.members ?? []).filter(m => m.kind === 'node').map(m => m.name),
    );
    if (enumVariantMembers.has(node.kind)) return 'node';
    if (node.kind === 'structFieldTypeNode') return 'node';

    return 'type_node';
}
