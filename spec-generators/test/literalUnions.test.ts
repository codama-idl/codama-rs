import { getSpec, type Spec } from '@codama/spec';
import { describe, expect, it } from 'vitest';

import { getReferencedLiteralUnions, literalUnionVariantName } from '../src/literalUnions';

const spec = getSpec();

describe('getReferencedLiteralUnions', () => {
    it('finds the two literalUnions (`base`, `isSigner`) and dedups the two value-identical `isSigner` refs', () => {
        const refs = getReferencedLiteralUnions(spec);
        expect(refs).toEqual([
            { typeName: 'Base', values: [2, 10] },
            { typeName: 'IsSigner', values: [true, false, 'either'] },
        ]);
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
                },
            ],
        };
        expect(() => getReferencedLiteralUnions(synthetic)).toThrow(/ambiguous|differently-named/);
    });

    it('dedups two same-named attributes carrying value-identical literalUnions into a single ref', () => {
        const isSignerAttributes = spec.categories
            .flatMap(c => c.nodes)
            .flatMap(n => n.attributes)
            .filter(a => a.name === 'isSigner' && a.type.kind === 'literalUnion');
        expect(isSignerAttributes).toHaveLength(2);
        expect(getReferencedLiteralUnions(spec).filter(r => r.typeName === 'IsSigner')).toHaveLength(1);
    });
});

describe('literalUnionVariantName', () => {
    it('maps booleans to True/False', () => {
        expect(literalUnionVariantName(true, 'IsSigner')).toBe('True');
        expect(literalUnionVariantName(false, 'IsSigner')).toBe('False');
    });

    it('PascalCases string values', () => {
        expect(literalUnionVariantName('either', 'IsSigner')).toBe('Either');
        expect(literalUnionVariantName('camelCase', 'IsSigner')).toBe('CamelCase');
    });

    it('prefixes numeric values with the type name', () => {
        expect(literalUnionVariantName(2, 'Base')).toBe('Base2');
        expect(literalUnionVariantName(10, 'Base')).toBe('Base10');
    });

    it('throws on numeric values that are not non-negative safe integers', () => {
        for (const value of [-1, 1.5, 1e21, Number.NaN]) {
            expect(() => literalUnionVariantName(value, 'Base')).toThrow(/non-negative safe integers/);
        }
    });
});
