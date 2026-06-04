import { getSpec } from '@codama/spec';
import { describe, expect, it } from 'vitest';

import { flattenNodeUnion, getEmittableUnions, getInlineUnionStripSuffix } from '../src/unions';

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

    it('skips HAND_WRITTEN_UNIONS even when they have a registered twin (e.g. value/valueNode)', () => {
        expect(getEmittableUnions(valueCategory, spec).map(u => u.name)).not.toContain('valueNode');
    });
});

describe('getInlineUnionStripSuffix', () => {
    it('returns the longest common PascalCase suffix shared by every flattened leaf', () => {
        const constantPdaSeedValue = pdaSeedCategory.unions.find(u => u.name === 'constantPdaSeedValue')!;
        // Leaves include `programIdValueNode` + every `valueNode` leaf
        // (all suffixed `ValueNode`).
        expect(getInlineUnionStripSuffix(constantPdaSeedValue, spec)).toBe('ValueNode');
    });

    it('handles a small inline union (enumValuePayload: structValueNode | tupleValueNode)', () => {
        const enumValuePayload = valueCategory.unions.find(u => u.name === 'enumValuePayload')!;
        expect(getInlineUnionStripSuffix(enumValuePayload, spec)).toBe('ValueNode');
    });
});

describe('flattenNodeUnion', () => {
    it('walks nested union members to leaf nodes', () => {
        // `linkNode` references `union(\'registeredLinkNode\')` which
        // lists 7 concrete link nodes.
        const linkNode = linkCategory.unions.find(u => u.name === 'linkNode')!;
        const kinds = flattenNodeUnion(linkNode, spec).map(n => n.kind);
        expect(kinds.toSorted()).toEqual([
            'accountLinkNode',
            'definedTypeLinkNode',
            'instructionAccountLinkNode',
            'instructionArgumentLinkNode',
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
            'instructionArgumentLinkNode',
            'instructionLinkNode',
            'pdaLinkNode',
            'programLinkNode',
        ]);
    });
});
