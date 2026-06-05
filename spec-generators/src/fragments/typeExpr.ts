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
            // Not used by any node attribute in the categories generated
            // today. The `topLevel.rootNode.standard` field uses it, but
            // `topLevel` isn't generated yet (PR #10). Render best-effort
            // so we don't hard-fail prematurely.
            return fragment`${JSON.stringify(expr.value)}`;
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
