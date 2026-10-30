import {
    address,
    anyNode,
    array,
    boolean,
    docs,
    enumeration,
    json,
    literal,
    literalUnion,
    node,
    string,
    stringDecimal,
    stringIdentifier,
    stringInteger,
    stringNamespace,
    stringPath,
    stringVersion,
    text,
    u32,
    union,
} from '@codama/spec/api';
import { describe, expect, it } from 'vitest';

import { getTypeExprFragment } from '../../src/fragments/typeExpr';

describe('getTypeExprFragment', () => {
    it('renders plain string as String', () => {
        expect(getTypeExprFragment(string()).content).toBe('String');
    });

    it('renders address as plain String (no brand, no import)', () => {
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

    it('renders json as the opaque `serde_json::Value` with a matching import', () => {
        const result = getTypeExprFragment(json());
        expect(result.content).toBe('Value');
        expect([...result.imports.keys()]).toEqual(['serde_json::Value']);
    });

    it("renders the `literal('codama')` TypeExpr as `String`", () => {
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

    it('routes stringVersion to plain String (no brand on the Rust side)', () => {
        const result = getTypeExprFragment(stringVersion());
        expect(result.content).toBe('String');
        expect(result.imports.size).toBe(0);
    });

    it('routes stringNamespace to Namespace via a crate-rooted import', () => {
        const result = getTypeExprFragment(stringNamespace());
        expect(result.content).toBe('Namespace');
        expect([...result.imports.keys()]).toEqual(['crate::Namespace']);
    });

    it('routes stringPath to Path via a crate-rooted import', () => {
        const result = getTypeExprFragment(stringPath());
        expect(result.content).toBe('Path');
        expect([...result.imports.keys()]).toEqual(['crate::Path']);
    });

    it('routes stringInteger to Integer via a crate-rooted import', () => {
        const result = getTypeExprFragment(stringInteger());
        expect(result.content).toBe('Integer');
        expect([...result.imports.keys()]).toEqual(['crate::Integer']);
    });

    it('routes stringDecimal to Decimal via a crate-rooted import', () => {
        const result = getTypeExprFragment(stringDecimal());
        expect(result.content).toBe('Decimal');
        expect([...result.imports.keys()]).toEqual(['crate::Decimal']);
    });

    it('routes text to Text via a crate-rooted import', () => {
        const result = getTypeExprFragment(text());
        expect(result.content).toBe('Text');
        expect([...result.imports.keys()]).toEqual(['crate::Text']);
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

    it('renders array(T) as Vec<T> and propagates the inner imports', () => {
        const result = getTypeExprFragment(array(node('pdaSeedValueNode')));
        expect(result.content).toBe('Vec<PdaSeedValueNode>');
        expect([...result.imports.keys()]).toEqual(['crate::PdaSeedValueNode']);
    });

    it('handles nested array types', () => {
        expect(getTypeExprFragment(array(array(boolean()))).content).toBe('Vec<Vec<bool>>');
    });
});
