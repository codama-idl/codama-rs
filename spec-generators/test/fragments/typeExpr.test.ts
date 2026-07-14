import {
    address,
    anyNode,
    array,
    boolean,
    docs,
    enumeration,
    f64,
    json,
    literal,
    literalUnion,
    nestedUnion,
    node,
    string,
    stringIdentifier,
    stringVersion,
    tuple,
    u32,
    union,
} from '@codama/spec/api';
import { describe, expect, it } from 'vitest';

import { getTypeExprFragment } from '../../src/fragments/typeExpr';

describe('getTypeExprFragment', () => {
    it('renders plain string as String', () => {
        expect(getTypeExprFragment(string()).content).toBe('String');
    });

    it('renders address as plain String for v1 (no brand, no import)', () => {
        const result = getTypeExprFragment(address());
        expect(result.content).toBe('String');
        expect(result.imports.size).toBe(0);
    });

    it('renders boolean as bool with no import', () => {
        const result = getTypeExprFragment(boolean());
        expect(result.content).toBe('bool');
        expect(result.imports.size).toBe(0);
    });

    it('renders integer widths to the matching Rust primitive', () => {
        expect(getTypeExprFragment(u32()).content).toBe('u32');
    });

    it('renders float as the bespoke `crate::Number` enum (mirrors JS `float → number`)', () => {
        // v1's only float (`numberValueNode.number`) is `f64`; Rust's
        // `Number` enum (`u64|i64|f64` with custom serde) is the
        // analogue of TS's polymorphic `number`.
        const result = getTypeExprFragment(f64());
        expect(result.content).toBe('Number');
        expect([...result.imports.keys()]).toEqual(['crate::Number']);
    });

    it('renders json as the opaque `serde_json::Value` with a matching import', () => {
        const result = getTypeExprFragment(json());
        expect(result.content).toBe('Value');
        expect([...result.imports.keys()]).toEqual(['serde_json::Value']);
    });

    it("renders the v1 `literal('codama')` TypeExpr as `String`", () => {
        expect(getTypeExprFragment(literal('codama')).content).toBe('String');
    });

    it('throws on a literalUnion in a nested position', () => {
        expect(() => getTypeExprFragment(literalUnion(1, 2))).toThrow(/literalUnion/);
    });

    it('routes stringIdentifier to CamelCaseString via a crate-rooted import', () => {
        const result = getTypeExprFragment(stringIdentifier());
        expect(result.content).toBe('CamelCaseString');
        expect([...result.imports.keys()]).toEqual(['crate::CamelCaseString']);
    });

    it('routes stringVersion to plain String (no Version brand on the Rust side in v1)', () => {
        const result = getTypeExprFragment(stringVersion());
        expect(result.content).toBe('String');
        expect(result.imports.size).toBe(0);
    });

    it('routes docs to Docs via a crate-rooted import', () => {
        const result = getTypeExprFragment(docs());
        expect(result.content).toBe('Docs');
        expect([...result.imports.keys()]).toEqual(['crate::Docs']);
    });

    it('routes enumeration references via PascalCase content + crate import', () => {
        const result = getTypeExprFragment(enumeration('endianness'));
        expect(result.content).toBe('Endianness');
        expect([...result.imports.keys()]).toEqual(['crate::Endianness']);
    });

    it('routes node references via PascalCase content + crate import', () => {
        const result = getTypeExprFragment(node('programLinkNode'));
        expect(result.content).toBe('ProgramLinkNode');
        expect([...result.imports.keys()]).toEqual(['crate::ProgramLinkNode']);
    });

    it('routes anyNode to the top-level crate::Node registry', () => {
        const result = getTypeExprFragment(anyNode());
        expect(result.content).toBe('Node');
        expect([...result.imports.keys()]).toEqual(['crate::Node']);
    });

    it('routes union references via PascalCase content when no override', () => {
        const result = getTypeExprFragment(union('typeNode'));
        expect(result.content).toBe('TypeNode');
        expect([...result.imports.keys()]).toEqual(['crate::TypeNode']);
    });

    it('renders nestedUnion(alias, kind) as Alias<Kind> with both crate imports', () => {
        const result = getTypeExprFragment(nestedUnion('nestedTypeNode', 'numberTypeNode'));
        expect(result.content).toBe('NestedTypeNode<NumberTypeNode>');
        const imports = [...result.imports.keys()].toSorted();
        expect(imports).toEqual(['crate::NestedTypeNode', 'crate::NumberTypeNode']);
    });

    it('renders array(T) as Vec<T> and propagates the inner imports', () => {
        const result = getTypeExprFragment(array(node('pdaSeedValueNode')));
        expect(result.content).toBe('Vec<PdaSeedValueNode>');
        expect([...result.imports.keys()]).toEqual(['crate::PdaSeedValueNode']);
    });

    it('handles nested array types', () => {
        expect(getTypeExprFragment(array(array(boolean()))).content).toBe('Vec<Vec<bool>>');
    });

    it('renders tuple(items) as (A, B, …) and merges the inner imports', () => {
        const result = getTypeExprFragment(tuple(node('programLinkNode'), boolean()));
        expect(result.content).toBe('(ProgramLinkNode, bool)');
        expect([...result.imports.keys()]).toEqual(['crate::ProgramLinkNode']);
    });
});
