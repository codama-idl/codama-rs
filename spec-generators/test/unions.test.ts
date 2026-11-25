import { getSpec } from '@codama/spec';
import { describe, expect, it } from 'vitest';

import {
    flattenNodeUnion,
    getEmittableUnions,
    getRegisteredOnlyLeafKinds,
    getUnionCategory,
    getUnionVariantName,
    isRegisteredCategoryUnion,
} from '../src/unions';

const spec = getSpec();
const linkCategory = spec.categories.find(c => c.name === 'link')!;
const pdaSeedCategory = spec.categories.find(c => c.name === 'pdaSeed')!;
const valueCategory = spec.categories.find(c => c.name === 'value')!;

describe('getEmittableUnions', () => {
    it('returns the category-main union (the standalone twin of a `registered…`) plus any referenced inline unions, sorted alphabetically', () => {
        // `pdaSeed` also has `constantPdaSeedValue` (inline,
        // referenced by `constantPdaSeedNode.value`), so both are
        // emittable; the sort puts `constantPdaSeedValue` first.
        expect(getEmittableUnions(linkCategory, spec).map(u => u.name)).toEqual(['linkNode']);
        expect(getEmittableUnions(pdaSeedCategory, spec).map(u => u.name)).toEqual([
            'constantPdaSeedValue',
            'pdaSeedNode',
        ]);
    });

    it('skips category-registry unions (`registered*`)', () => {
        expect(getEmittableUnions(linkCategory, spec).map(u => u.name)).not.toContain('registeredLinkNode');
    });

    it('skips inline unions that are not referenced anywhere in the spec', () => {
        // The derived rule only emits an inline union if at least one
        // node attribute references it. linkCategory has no inline
        // members at all, so the rule yields just `linkNode`.
        const names = getEmittableUnions(linkCategory, spec).map(u => u.name);
        for (const u of linkCategory.unions) {
            if (u.name.startsWith('registered') || u.name === 'linkNode') continue;
            expect(names).not.toContain(u.name);
        }
    });

    it('emits the `value` category union (handled by the RegisteredNodes renderer, dispatched in `index.ts`)', () => {
        expect(getEmittableUnions(valueCategory, spec).map(u => u.name)).toContain('valueNode');
    });
});

describe('isRegisteredCategoryUnion', () => {
    it('is true when the registered<X> twin has extra members (e.g. value/valueNode)', () => {
        const valueNode = valueCategory.unions.find(u => u.name === 'valueNode')!;
        expect(isRegisteredCategoryUnion(valueNode, spec)).toBe(true);
    });

    it('is false when registered<X> mirrors the standalone (e.g. link/linkNode, pdaSeed/pdaSeedNode)', () => {
        const linkNode = linkCategory.unions.find(u => u.name === 'linkNode')!;
        const pdaSeedNode = pdaSeedCategory.unions.find(u => u.name === 'pdaSeedNode')!;
        expect(isRegisteredCategoryUnion(linkNode, spec)).toBe(false);
        expect(isRegisteredCategoryUnion(pdaSeedNode, spec)).toBe(false);
    });
});

describe('getRegisteredOnlyLeafKinds', () => {
    it('returns the kinds present in registered<X> but absent from standalone <X>, in spec order', () => {
        const valueNode = valueCategory.unions.find(u => u.name === 'valueNode')!;
        // spec `registeredValueNode` adds `mapEntryValueNode` then
        // `structFieldValueNode` after the standalone members.
        expect(getRegisteredOnlyLeafKinds(valueNode, spec)).toEqual(['mapEntryValueNode', 'structFieldValueNode']);
    });

    it('returns an empty list for unions whose registered twin matches the standalone', () => {
        const linkNode = linkCategory.unions.find(u => u.name === 'linkNode')!;
        expect(getRegisteredOnlyLeafKinds(linkNode, spec)).toEqual([]);
    });
});

describe('getUnionVariantName', () => {
    it('drops the trailing Node, like the flat Node enum', () => {
        expect(getUnionVariantName('integerValueNode')).toBe('IntegerValue');
        expect(getUnionVariantName('amountNumberDisplayNode')).toBe('AmountNumberDisplay');
    });

    it('drops the category word within the main union of a category', () => {
        expect(getUnionVariantName('accountLinkNode', 'link')).toBe('Account');
        expect(getUnionVariantName('fixedCountNode', 'count')).toBe('Fixed');
        expect(getUnionVariantName('integerValueNode', 'value')).toBe('Integer');
        expect(getUnionVariantName('fixedSizeTransformNode', 'transform')).toBe('FixedSize');
    });

    it('keeps the whole name when it does not end with the category word', () => {
        expect(getUnionVariantName('accountValueNode', 'contextualValue')).toBe('AccountValue');
    });
});

describe('getUnionCategory', () => {
    it('finds the category whose main union a union is', () => {
        const linkNode = linkCategory.unions.find(u => u.name === 'linkNode')!;
        expect(getUnionCategory(linkNode, spec)).toBe('link');
        const valueNode = valueCategory.unions.find(u => u.name === 'valueNode')!;
        expect(getUnionCategory(valueNode, spec)).toBe('value');
    });

    it('gives nothing for inline unions, whatever their suffix', () => {
        const constantPdaSeedValue = pdaSeedCategory.unions.find(u => u.name === 'constantPdaSeedValue')!;
        expect(getUnionCategory(constantPdaSeedValue, spec)).toBeUndefined();
        const injectableInteger = valueCategory.unions.find(u => u.name === 'injectableIntegerValueNode')!;
        expect(getUnionCategory(injectableInteger, spec)).toBeUndefined();
    });
});

describe('flattenNodeUnion', () => {
    it('walks nested union members to leaf nodes', () => {
        const linkNode = linkCategory.unions.find(u => u.name === 'linkNode')!;
        const kinds = flattenNodeUnion(linkNode, spec).map(n => n.kind);
        expect(kinds.toSorted()).toEqual([
            'accountLinkNode',
            'definedTypeLinkNode',
            'instructionAccountLinkNode',
            'instructionLinkNode',
            'pdaLinkNode',
            'programLinkNode',
        ]);
    });

    it('returns direct node members unchanged when the union has no nested unions', () => {
        const registered = linkCategory.unions.find(u => u.name === 'registeredLinkNode')!;
        const kinds = flattenNodeUnion(registered, spec).map(n => n.kind);
        expect(kinds.toSorted()).toEqual([
            'accountLinkNode',
            'definedTypeLinkNode',
            'instructionAccountLinkNode',
            'instructionLinkNode',
            'pdaLinkNode',
            'programLinkNode',
        ]);
    });
});
