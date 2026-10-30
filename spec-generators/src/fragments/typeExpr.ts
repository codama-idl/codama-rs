import { pascalCase } from '@codama/fragments';
import { type Fragment, fragment } from '@codama/fragments/rust';
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
 * Mapping:
 *
 *   - `address`                   → `String`
 *   - `string`                    → see {@link getStringExprFragment}
 *   - `boolean`                   → `bool`
 *   - `integer`                   → `uN` / `iN` by width
 *   - `json`                      → `serde_json::Value` (opaque, arbitrary JSON)
 *   - `docs`                      → `Docs`
 *   - `text`                      → `Text` (`string | textNode`)
 *   - `enumeration('foo')`        → `Foo`
 *   - `node('fooBar')`            → `FooBar`
 *   - `anyNode`                   → `crate::Node` (the top-level registry)
 *   - `union('fooBar')`           → `FooBar`
 *   - `array(of)`                 → `Vec<Of>`
 */
export function getTypeExprFragment(expr: TypeExpr): Fragment {
    switch (expr.kind) {
        case 'address':
            return fragment`String`;
        case 'string':
            return getStringExprFragment(expr);
        case 'boolean':
            return fragment`bool`;
        case 'integer': {
            const rust = NUMBER_FORMAT_TO_RUST.get(expr.width);
            if (!rust) throw new Error(`unknown integer width "${expr.width}"`);
            return fragment`${rust}`;
        }
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
            // type expression (e.g. `array(literalUnion(...))`), and the nested
            // position gives us no name source.
            throw new Error('literalUnion TypeExpr is only supported at the top level of a node attribute');
        case 'codamaVersion':
            return fragment`String`;
        case 'docs':
            return use('crate::Docs');
        case 'text':
            return use('crate::Text');
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
        case 'array': {
            const inner = getTypeExprFragment(expr.of);
            return fragment`Vec<${inner}>`;
        }
    }
}

/**
 * The Rust type for a `string` type expression, by constraint:
 *
 *   - none, `namespace`, `version`         -> `String`
 *   - `identifier`                         -> `CamelCaseString`
 *   - `integer`                            -> `Integer` (unsigned or signed, up to 128 bits)
 *   - `decimal`                            -> `Decimal` (a decimal number written as a string)
 *   - `path`                               -> `Path` (a path expression such as `header.kind`)
 */
function getStringExprFragment(expr: Extract<TypeExpr, { kind: 'string' }>): Fragment {
    switch (expr.constraint) {
        case 'identifier':
            return use('crate::CamelCaseString');
        case 'integer':
            return use('crate::Integer');
        case 'decimal':
            return use('crate::Decimal');
        case 'path':
            return use('crate::Path');
        case 'namespace':
        case 'version':
        case undefined:
            return fragment`String`;
    }
}
