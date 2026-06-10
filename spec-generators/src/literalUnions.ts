import { pascalCase } from '@codama/fragments';
import type { Spec } from '@codama/spec';

/**
 * A primitive literal value, matching the spec's `LiteralValue`. We
 * keep a local copy so this module doesn't need a structural import
 * from `@codama/spec`'s public types — the runtime shape is identical.
 */
type LiteralValue = boolean | number | string;

/**
 * One distinct `literalUnion` discovered in the spec, paired with the
 * type name the generator will emit for it.
 *
 * A `literalUnion` TypeExpr is anonymous and inline in v1: it has no
 * name slot of its own, so the generated Rust type name is derived
 * from the **attribute** that references it
 * (`pascalCase(attr.name)`). Distinct value-sets become distinct
 * types; identical value-sets (e.g. the two `isSigner` references)
 * collapse to a single type.
 */
export interface LiteralUnionTypeRef {
    /** The Rust type name, `pascalCase` of the referencing attribute. */
    readonly typeName: string;
    /** The literalUnion's values, in spec declaration order. */
    readonly values: readonly LiteralValue[];
}

/**
 * Walk every node attribute in the spec, collect every `literalUnion`
 * TypeExpr, dedup by value-set, and pair each distinct value-set with
 * the type name derived from its referencing attribute.
 *
 * The walk is recursive (mirrors `getReferencedUnionNames`): a
 * `literalUnion` nested inside an `array.of` / `tuple.items` would be
 * found, even though no such case exists in v1.
 *
 * Throws when the same value-set is referenced by two
 * differently-named attributes — that would make the derived type
 * name ambiguous. This case doesn't occur in v1 (the sole literalUnion
 * is `isSigner` used by two `isSigner` attributes), but the guard
 * keeps the contract explicit so a future spec regression surfaces.
 */
export function getReferencedLiteralUnions(spec: Spec): readonly LiteralUnionTypeRef[] {
    const byValueKey = new Map<string, { values: readonly LiteralValue[]; attrName: string }>();

    const walk = (t: unknown, attrName: string): void => {
        if (!t || typeof t !== 'object') return;
        const node = t as { kind?: string; values?: readonly LiteralValue[] };
        if (node.kind === 'literalUnion' && Array.isArray(node.values)) {
            const key = JSON.stringify(node.values);
            const existing = byValueKey.get(key);
            if (existing && existing.attrName !== attrName) {
                throw new Error(
                    `literalUnion value-set ${key} is referenced by two differently-named attributes ("${existing.attrName}" and "${attrName}"). The generated type name would be ambiguous; either rename one attribute or model the literalUnion as a named registry entry upstream.`,
                );
            }
            if (!existing) byValueKey.set(key, { values: node.values, attrName });
            return;
        }
        for (const v of Object.values(t)) walk(v, attrName);
    };

    for (const category of spec.categories) {
        for (const node of category.nodes) {
            for (const attr of node.attributes) walk(attr.type, attr.name);
        }
    }

    return [...byValueKey.values()]
        .map(({ values, attrName }) => ({ typeName: pascalCase(attrName), values }))
        .toSorted((a, b) => a.typeName.localeCompare(b.typeName));
}

/**
 * The Rust variant name for a single literalUnion value.
 *
 *   - `true`     → `True`
 *   - `false`    → `False`
 *   - `"either"` → `Either` (string values are PascalCased).
 *
 * Numeric values would also PascalCase via their string form, but no
 * v1 literalUnion uses numbers.
 */
export function literalUnionVariantName(value: LiteralValue): string {
    if (typeof value === 'boolean') return value ? 'True' : 'False';
    if (typeof value === 'string') return pascalCase(value);
    return pascalCase(String(value));
}
