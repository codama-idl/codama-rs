import { pascalCase } from '@codama/fragments';
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
 * for. A union is "emittable" when:
 *
 *   - It has a `registered<PascalCase>` sibling in the same category —
 *     i.e. it's the category's main union (the standalone twin of a
 *     registered/dispatch union); OR
 *   - It's an inline union per {@link isInlineUnion}: no `registered`
 *     twin AND referenced by at least one node attribute somewhere
 *     in the spec. This rule is derived from the spec; no hand-list.
 *
 * Sorted alphabetically by name for stable output.
 */
export function getEmittableUnions(category: Spec['categories'][number], spec: Spec): readonly UnionSpec[] {
    const referenced = getReferencedUnionNames(spec);
    const allUnionNames = new Set(spec.categories.flatMap(c => c.unions).map(u => u.name));
    return category.unions
        .filter(u => !u.name.startsWith(REGISTERED_UNION_PREFIX))
        .filter(u => hasRegisteredTwin(u.name, allUnionNames) || isInlineUnion(u, allUnionNames, referenced))
        .toSorted((a, b) => a.name.localeCompare(b.name));
}

/**
 * `true` when the standalone `union` is the twin of a
 * `registered<PascalCase>` that contains members the standalone
 * doesn't (i.e. the `registered<X>` has at least one
 * `#[registered]`-only variant). Such unions need the
 * `#[derive(RegisteredNodes)]` emission mode — see
 * `registeredUnionPage.ts`. Categories like `link`/`count` whose
 * registered twin is identical to the standalone do NOT match.
 */
export function isRegisteredCategoryUnion(union: UnionSpec, spec: Spec): boolean {
    const twin = spec.categories
        .flatMap(c => c.unions)
        .find(u => u.name === `${REGISTERED_UNION_PREFIX}${pascalCase(union.name)}`);
    if (!twin) return false;
    const standaloneKinds = new Set([...flattenNodeUnion(union, spec)].map(n => n.kind));
    const twinKinds = [...flattenNodeUnion(twin, spec)].map(n => n.kind);
    return twinKinds.some(k => !standaloneKinds.has(k));
}

/**
 * The leaf node kinds that are present in the `registered<X>` twin
 * but NOT in the standalone `<X>` — i.e. the variants that must be
 * marked `#[registered]` in the emitted enum. Returned in spec
 * declaration order (the `registered<X>` union's member order),
 * since that order is deterministic and has no semantic effect.
 */
export function getRegisteredOnlyLeafKinds(union: UnionSpec, spec: Spec): readonly string[] {
    const twin = spec.categories
        .flatMap(c => c.unions)
        .find(u => u.name === `${REGISTERED_UNION_PREFIX}${pascalCase(union.name)}`);
    if (!twin) return [];
    const standaloneKinds = new Set([...flattenNodeUnion(union, spec)].map(n => n.kind));
    return [...flattenNodeUnion(twin, spec)].map(n => n.kind).filter(k => !standaloneKinds.has(k));
}

/**
 * `true` when `union` is an inline / synthetic union: it has no
 * `registered<PascalCase>` sibling anywhere in the spec AND it is
 * actually referenced by some node attribute (we only emit unions
 * that are used). Derived purely from the spec structure — no
 * hand-maintained allowlist.
 */
export function isInlineUnion(
    union: UnionSpec,
    allUnionNames: ReadonlySet<string>,
    referenced: ReadonlySet<string>,
): boolean {
    if (hasRegisteredTwin(union.name, allUnionNames)) return false;
    return referenced.has(union.name);
}

function hasRegisteredTwin(unionName: string, allUnionNames: ReadonlySet<string>): boolean {
    return allUnionNames.has(`${REGISTERED_UNION_PREFIX}${pascalCase(unionName)}`);
}

/**
 * Every spec union name reachable from at least one node attribute
 * (recursively through `array(of)` / `tuple(items)` / etc.). Used by
 * {@link isInlineUnion} so we only emit unions that something
 * actually references.
 */
export function getReferencedUnionNames(spec: Spec): ReadonlySet<string> {
    const referenced = new Set<string>();
    const walk = (t: unknown): void => {
        if (!t || typeof t !== 'object') return;
        const node = t as { kind?: string; name?: string };
        if (node.kind === 'union' && typeof node.name === 'string') referenced.add(node.name);
        for (const v of Object.values(t)) walk(v);
    };
    for (const cat of spec.categories) {
        for (const n of cat.nodes) {
            for (const a of n.attributes) walk(a.type);
        }
    }
    return referenced;
}

/**
 * The PascalCase suffix to strip from each leaf node's kind when
 * deriving variant names for an inline union. Computed as the
 * longest common PascalCase suffix shared by every leaf's
 * `pascalCase(kind)`, trimmed back to start at an uppercase letter
 * so we never strip mid-word.
 *
 *   - `constantPdaSeedValue` (15 leaves)  → `'ValueNode'`
 *   - `enumValuePayload`     (2 leaves)   → `'ValueNode'`
 *   - `pdaValuePda`          (2 leaves)   → `'Node'`
 *
 * For category-main unions (those with a `registered<X>` twin) the
 * stripped suffix is the union's own pascalCase name — handled in
 * {@link variantStripSuffix} of `unionPage.ts`.
 */
export function getInlineUnionStripSuffix(union: UnionSpec, spec: Spec): string {
    const leaves = [...flattenNodeUnion(union, spec)].map(n => pascalCase(n.kind));
    if (leaves.length === 0) return '';
    let suffix = '';
    const minLen = Math.min(...leaves.map(s => s.length));
    for (let i = 1; i <= minLen; i++) {
        const ch = leaves[0][leaves[0].length - i];
        if (!leaves.every(s => s[s.length - i] === ch)) break;
        suffix = ch + suffix;
    }
    // Trim back so we always start at an uppercase letter (word boundary).
    for (let i = 0; i < suffix.length; i++) {
        const ch = suffix[i];
        if (ch >= 'A' && ch <= 'Z') return suffix.slice(i);
    }
    return '';
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
