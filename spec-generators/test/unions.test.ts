import { getSpec } from '@codama/spec';
import { describe, expect, it } from 'vitest';

import { flattenNodeUnion, getEmittableUnions } from '../src/unions';

const spec = getSpec();
const linkCategory = spec.categories.find(c => c.name === 'link')!;

describe('getEmittableUnions', () => {
    it('returns spec unions whose name does NOT start with `registered`, sorted alphabetically', () => {
        const names = getEmittableUnions(linkCategory).map(u => u.name);
        expect(names).toEqual(['linkNode']);
    });

    it('skips category-registry unions (`registered*`)', () => {
        const names = getEmittableUnions(linkCategory).map(u => u.name);
        expect(names).not.toContain('registeredLinkNode');
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
