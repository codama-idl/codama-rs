import { getSpec } from '@codama/spec';
import { describe, expect, it } from 'vitest';

import { flattenNodeUnion, getEmittableUnions } from '../src/unions';

const spec = getSpec();
const linkCategory = spec.categories.find(c => c.name === 'link')!;
const pdaSeedCategory = spec.categories.find(c => c.name === 'pdaSeed')!;

describe('getEmittableUnions', () => {
    it('returns the category-main union (the standalone twin of a `registered…`), sorted alphabetically', () => {
        // `pdaSeed` also has `constantPdaSeedValue` in INLINE_UNIONS,
        // so both are emittable; the sort puts `constantPdaSeedValue`
        // before `pdaSeedNode`.
        expect(getEmittableUnions(linkCategory).map(u => u.name)).toEqual(['linkNode']);
        expect(getEmittableUnions(pdaSeedCategory).map(u => u.name)).toEqual(['constantPdaSeedValue', 'pdaSeedNode']);
    });

    it('skips category-registry unions (`registered*`)', () => {
        expect(getEmittableUnions(linkCategory).map(u => u.name)).not.toContain('registeredLinkNode');
    });

    it('skips inline / synthetic unions that are NOT in the INLINE_UNIONS allowlist', () => {
        // `linkNode`'s category has no inline-union members, so we can
        // just confirm no spurious emission. A category with inline
        // unions out of the allowlist would also be filtered out (no
        // such case in pdaSeed today — constantPdaSeedValue IS in the
        // allowlist, so it appears).
        const names = getEmittableUnions(linkCategory).map(u => u.name);
        for (const u of linkCategory.unions) {
            if (u.name.startsWith('registered')) continue;
            if (u.name === 'linkNode') continue;
            expect(names).not.toContain(u.name);
        }
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
