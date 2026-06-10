import { getSpec, type Spec } from '@codama/spec';
import { describe, expect, it } from 'vitest';

import { getReferencedLiteralUnions, literalUnionVariantName } from '../src/literalUnions';

const spec = getSpec();

describe('getReferencedLiteralUnions', () => {
    it('finds the sole v1 literalUnion (`isSigner`) and dedups the two value-identical refs into one entry', () => {
        const refs = getReferencedLiteralUnions(spec);
        expect(refs).toHaveLength(1);
        expect(refs[0].typeName).toBe('IsSigner');
        expect(refs[0].values).toEqual([true, false, 'either']);
    });

    it('returns refs sorted alphabetically by typeName', () => {
        // Synthesise a spec with two distinct literalUnions on differently-named attributes;
        // the result must be sorted by typeName.
        const synthetic: Spec = {
            version: '1.0.0',
            categories: [
                {
                    name: 'fake',
                    docs: [],
                    nodes: [
                        {
                            kind: 'fakeNode',
                            docs: [],
                            attributes: [
                                { name: 'zebra', type: { kind: 'literalUnion', values: ['a', 'b'] } },
                                { name: 'apple', type: { kind: 'literalUnion', values: [1, 2] } },
                            ],
                            examples: [],
                        },
                    ],
                    unions: [],
                    enumerations: [],
                    nestedUnions: [],
                },
            ],
        };
        const refs = getReferencedLiteralUnions(synthetic);
        expect(refs.map(r => r.typeName)).toEqual(['Apple', 'Zebra']);
    });

    it('throws when the same value-set is referenced by two differently-named attributes (would yield an ambiguous type name)', () => {
        const synthetic: Spec = {
            version: '1.0.0',
            categories: [
                {
                    name: 'fake',
                    docs: [],
                    nodes: [
                        {
                            kind: 'fakeNode',
                            docs: [],
                            attributes: [
                                { name: 'alpha', type: { kind: 'literalUnion', values: [true, false] } },
                                { name: 'beta', type: { kind: 'literalUnion', values: [true, false] } },
                            ],
                            examples: [],
                        },
                    ],
                    unions: [],
                    enumerations: [],
                    nestedUnions: [],
                },
            ],
        };
        expect(() => getReferencedLiteralUnions(synthetic)).toThrow(/ambiguous|differently-named/);
    });

    it('dedups two same-named attributes carrying value-identical literalUnions into a single ref', () => {
        // Mirrors the v1 isSigner case: two attributes both named
        // `isSigner` carrying `[true, false, "either"]` -> one IsSigner.
        const synthetic: Spec = {
            version: '1.0.0',
            categories: [
                {
                    name: 'fake',
                    docs: [],
                    nodes: [
                        {
                            kind: 'fooNode',
                            docs: [],
                            attributes: [
                                { name: 'isSigner', type: { kind: 'literalUnion', values: [true, false, 'either'] } },
                            ],
                            examples: [],
                        },
                        {
                            kind: 'barNode',
                            docs: [],
                            attributes: [
                                { name: 'isSigner', type: { kind: 'literalUnion', values: [true, false, 'either'] } },
                            ],
                            examples: [],
                        },
                    ],
                    unions: [],
                    enumerations: [],
                    nestedUnions: [],
                },
            ],
        };
        const refs = getReferencedLiteralUnions(synthetic);
        expect(refs).toHaveLength(1);
        expect(refs[0].typeName).toBe('IsSigner');
    });
});

describe('literalUnionVariantName', () => {
    it('maps booleans to True/False', () => {
        expect(literalUnionVariantName(true)).toBe('True');
        expect(literalUnionVariantName(false)).toBe('False');
    });

    it('PascalCases string values', () => {
        expect(literalUnionVariantName('either')).toBe('Either');
        expect(literalUnionVariantName('camelCase')).toBe('CamelCase');
    });

    it('handles numeric values via their string form', () => {
        expect(literalUnionVariantName(42)).toBe('42');
    });
});
