import {
    address,
    array,
    boolean,
    docs,
    enumeration,
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

    it('renders a string literal as a JSON-quoted source-form literal', () => {
        expect(getTypeExprFragment(literal('codama')).content).toBe('"codama"');
    });

    it('renders a boolean literal', () => {
        expect(getTypeExprFragment(literal(true)).content).toBe('true');
    });

    it('throws on literalUnion (only used inside spec enumerations, not as a node attribute type)', () => {
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

    it('routes enumeration references through the override table when present', () => {
        const result = getTypeExprFragment(enumeration('endianness'));
        // The Rust name `Endian` comes from ENUMERATION_NAME_OVERRIDES.
        expect(result.content).toBe('Endian');
        expect([...result.imports.keys()]).toEqual(['crate::Endian']);
    });

    it('routes node references via PascalCase content + crate import', () => {
        const result = getTypeExprFragment(node('programLinkNode'));
        expect(result.content).toBe('ProgramLinkNode');
        expect([...result.imports.keys()]).toEqual(['crate::ProgramLinkNode']);
    });

    it('routes union references through UNION_NAME_OVERRIDES when present', () => {
        const result = getTypeExprFragment(union('pdaValuePda'));
        expect(result.content).toBe('PdaValue');
        expect([...result.imports.keys()]).toEqual(['crate::PdaValue']);
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
