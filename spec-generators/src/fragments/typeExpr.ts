import { pascalCase } from '@codama/fragments';
import { type Fragment, fragment, mergeFragments } from '@codama/fragments/rust';
import type { TypeExpr } from '@codama/spec';

import { ENUMERATION_NAME_OVERRIDES, UNION_NAME_OVERRIDES } from '../defaults';
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

const FLOAT_WIDTH_TO_RUST: ReadonlyMap<string, string> = new Map([
    ['f32', 'f32'],
    ['f64', 'f64'],
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
 *   - `integer`/`float`           → `uN` / `iN` / `fN` by width
 *   - `docs`                      → `Docs`
 *   - `enumeration('foo')`        → `Foo` (subject to overrides)
 *   - `node('fooBar')`            → `FooBar`
 *   - `union('fooBar')`           → `FooBar` (subject to overrides)
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
        case 'float': {
            const rust = FLOAT_WIDTH_TO_RUST.get(expr.width);
            if (!rust) throw new Error(`unknown float width "${expr.width}"`);
            return fragment`${rust}`;
        }
        case 'literal':
            // Not used directly in any v1 node attribute; render best-effort.
            return fragment`${JSON.stringify(expr.value)}`;
        case 'literalUnion':
            // Only `shared` enumerations use this directly, and we don't generate
            // enumerations here. Fail loudly so a regression surfaces.
            throw new Error('literalUnion TypeExpr is not supported at the node-attribute level in v1');
        case 'codamaVersion':
            // No Rust analogue in v1; render as `String`.
            return fragment`String`;
        case 'docs':
            return use('crate::Docs');
        case 'enumeration':
            return use(`crate::${ENUMERATION_NAME_OVERRIDES.get(expr.name) ?? pascalCase(expr.name)}`);
        case 'node':
            return use(`crate::${pascalCase(expr.name)}`);
        case 'union':
            return use(`crate::${UNION_NAME_OVERRIDES.get(expr.name) ?? pascalCase(expr.name)}`);
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
