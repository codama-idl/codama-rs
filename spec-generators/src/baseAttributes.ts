import type { NodeSpec, Spec } from '@codama/spec';

/**
 * Return a copy of `spec` with `spec.base.attributes` appended to every node, after its declared attributes.
 * The spec requires base attributes (for example `plugins`) to serialise last on every node.
 * With no base attributes, the function returns `spec` unchanged.
 */
export function withBaseAttributes(spec: Spec): Spec {
    const baseAttributes = spec.base?.attributes ?? [];
    if (baseAttributes.length === 0) return spec;
    return {
        ...spec,
        categories: spec.categories.map(category => ({
            ...category,
            nodes: category.nodes.map(
                (node): NodeSpec => ({
                    ...node,
                    attributes: [...node.attributes, ...baseAttributes],
                }),
            ),
        })),
    };
}
