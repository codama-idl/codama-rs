import { pascalCase } from '@codama/fragments';
import { type Fragment, fragment, mergeFragments } from '@codama/fragments/rust';
import type { TypeExpr } from '@codama/spec';

import { use } from './helpers';

const NUMBER_FORMAT_TO_RUST: ReadonlyMap<string, string> = new Map([
    ['u8', 'u8'],
    ['u16', 'u16'],
    ['u32', 'u32'],
    ['u64', 'u64'],
    ['u128', 'u128'],
    ['i8', 'i8'],
    ['i16', 'i16'],
    ['i32', 'i32'],
    ['i64', 'i64'],
    ['i128', 'i128'],
]);

/**
 * Translate a spec {@link TypeExpr} to a Rust type expression. The
 * fragment's content is the rendered type text (e.g.
 * `Option<ProgramLinkNode>`) and its import map carries the
 * `crate::<name>` paths the surrounding file needs.
 *
 * v1 mapping:
 *
 *   - `address`                   → `String`
 *   - `string` (no constraint)    → `String`
 *   - `string` (`identifier`)     → `CamelCaseString`
 *   - `string` (`version`)        → `String`
 *   - `boolean`                   → `bool`
 *   - `integer`                   → `uN` / `iN` by width
 *   - `float`                     → `crate::Number` (bespoke u64|i64|f64 enum)
 *   - `json`                      → `serde_json::Value` (opaque, arbitrary JSON)
 *   - `docs`                      → `Docs`
 *   - `enumeration('foo')`        → `Foo`
 *   - `node('fooBar')`            → `FooBar`
 *   - `anyNode`                   → `crate::Node` (the top-level registry)
 *   - `union('fooBar')`           → `FooBar`
 *   - `nestedUnion('alias','k')`  → `Alias<Kind>` (only `nestedTypeNode` in v1)
 *   - `array(of)`                 → `Vec<Of>`
 *   - `tuple(items)`              → `(A, B, …)`
 */
export function getTypeExprFragment(expr: TypeExpr): Fragment {
    switch (expr.kind) {
        case 'address':
            return fragment`String`;
        case 'string':
            if (expr.constraint === 'identifier') return use('crate::CamelCaseString');
            return fragment`String`;
        case 'boolean':
            return fragment`bool`;
        case 'integer': {
            const rust = NUMBER_FORMAT_TO_RUST.get(expr.width);
            if (!rust) throw new Error(`unknown integer width "${expr.width}"`);
            return fragment`${rust}`;
        }
        case 'float':
            // v1's only float (`numberValueNode.number`) is `f64`; Rust uses
            // the bespoke `Number` enum (`u64|i64|f64` with custom serde +
            // `From<uN/iN/fN>` impls). Mirrors the JS generator, which maps
            // `float → number` (TS's polymorphic number type).
            return use('crate::Number');
        case 'json':
            // An opaque, arbitrary JSON value. The spec deliberately leaves
            // its shape undescribed, so it maps to `serde_json::Value`.
            return use('serde_json::Value');
        case 'literal':
            // The literal value lives in the hand-written `Default`.
            return fragment`String`;
        case 'literalUnion':
            // `literalUnion` at the top level of an attribute is resolved by
            // `attributeBodyLine.ts` (the attribute name supplies the type name).
            // Reaching this case means a literalUnion is nested inside another
            // type expression (e.g. `array(literalUnion(...))`) — that doesn't
            // occur in v1, and the nested position gives us no name source.
            // Fail loudly so a future spec regression surfaces here.
            throw new Error('literalUnion TypeExpr is only supported at the top level of a node attribute');
        case 'codamaVersion':
            // No Rust analogue in v1; render as `String`.
            return fragment`String`;
        case 'docs':
            return use('crate::Docs');
        case 'enumeration':
            return use(`crate::${pascalCase(expr.name)}`);
        case 'node':
            return use(`crate::${pascalCase(expr.name)}`);
        case 'anyNode':
            // A field that holds any node at all maps to the top-level
            // `crate::Node` registry, the enum over every node kind.
            return use('crate::Node');
        case 'union':
            return use(`crate::${pascalCase(expr.name)}`);
        case 'nestedUnion': {
            // v1 only has one nested-union alias: `nestedTypeNode`. The
            // Rust type alias is `NestedTypeNode<Kind>`.
            return fragment`${use(`crate::${pascalCase(expr.alias)}`)}<${use(`crate::${pascalCase(expr.name)}`)}>`;
        }
        case 'array': {
            const inner = getTypeExprFragment(expr.of);
            return fragment`Vec<${inner}>`;
        }
        case 'tuple': {
            const items = expr.items.map(getTypeExprFragment);
            const joined = mergeFragments(items, contents => contents.join(', '));
            return fragment`(${joined})`;
        }
    }
}
