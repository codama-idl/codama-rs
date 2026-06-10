import type { NodeSpec, Spec, UnionSpec } from '@codama/spec';

/**
 * Spec unions starting with `registered` are category-registry unions
 * (e.g. `registeredLinkNode`); the Rust crate exposes one flattened
 * enum per category instead, so we skip them and recurse through them
 * via {@link flattenNodeUnion} when they appear as nested members.
 */
const REGISTERED_UNION_PREFIX = 'registered';

/**
 * The spec unions in a category that the generator emits Rust enums
 * for, sorted alphabetically by name for stable output.
 */
export function getEmittableUnions(category: Spec['categories'][number]): readonly UnionSpec[] {
    return category.unions
        .filter(u => !u.name.startsWith(REGISTERED_UNION_PREFIX))
        .toSorted((a, b) => a.name.localeCompare(b.name));
}

/**
 * Walk a union's members, recursively expanding nested `union(...)`
 * references down to their leaf nodes. Returns the flat list of
 * concrete node specs in spec declaration order.
 *
 * `nestedUnion` members are not followed (they're name-aliased and
 * break the cycle Rust-side). Unknown member references are skipped
 * silently — the spec validator catches those upstream.
 *
 * `@codama/spec` doesn't expose a flatten helper of its own, so the
 * traversal is implemented here against the explicit `spec` arg.
 */
export function flattenNodeUnion(union: UnionSpec, spec: Spec): readonly NodeSpec[] {
    const unionByName = new Map(spec.categories.flatMap(c => c.unions).map(u => [u.name, u]));
    const nodeByKind = new Map(spec.categories.flatMap(c => c.nodes).map(n => [n.kind, n]));
    const out: NodeSpec[] = [];
    const visited = new Set<string>();
    const stack: string[] = [union.name];
    while (stack.length > 0) {
        const name = stack.pop();
        if (name === undefined || visited.has(name)) continue;
        visited.add(name);
        const u = unionByName.get(name);
        if (!u) continue;
        for (const m of u.members) {
            if (m.kind === 'node') {
                const node = nodeByKind.get(m.name);
                if (node) out.push(node);
            } else if (m.kind === 'union') {
                stack.push(m.name);
            }
        }
    }
    return out;
}
