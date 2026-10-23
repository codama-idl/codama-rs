import type { NodeSpec, Spec } from '@codama/spec';

import { flattenNodeUnion } from './unions';

/**
 * Which `codama_nodes_derive` macro a generated node struct uses.
 *
 *   - `node`: the default. Used by every non-`type`-category node and by `type`-category nodes outside the
 *     `typeNode` union: `enumVariantTypeNode` and `structFieldTypeNode`.
 *   - `type_node`: a `type`-category node that is a leaf of the `typeNode` union.
 *     Expands to `#[node] + #[derive(TypeNode)]`.
 */
export type NodeMacroFlavor = 'node' | 'type_node';

/**
 * Determine which derive macro to use for a generated node struct.
 *
 * Rules (spec-derived):
 *
 *   - `type`-category node that is a leaf of the `typeNode` union → `type_node`.
 *   - any other `type`-category node → `node`. It routes through `Node::Type(...)` like a regular `#[node]`.
 *   - any other category → `node`. The category check keeps `definedTypeLinkNode`, a `typeNode` leaf, on `#[node]`.
 */
export function getNodeMacroFlavor(node: NodeSpec, spec: Spec): NodeMacroFlavor {
    const typeCat = spec.categories.find(c => c.name === 'type');
    if (!typeCat) return 'node';
    const typeKinds = new Set(typeCat.nodes.map(n => n.kind));
    if (!typeKinds.has(node.kind)) return 'node';

    const typeNodeUnion = typeCat.unions.find(u => u.name === 'typeNode');
    if (!typeNodeUnion) {
        throw new Error(
            `The spec has a "type" category but no "typeNode" union, so the macro flavor of "${node.kind}" is unknown.`,
        );
    }
    return flattenNodeUnion(typeNodeUnion, spec).some(n => n.kind === node.kind) ? 'type_node' : 'node';
}
