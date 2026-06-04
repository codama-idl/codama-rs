import { getSpec } from '@codama/spec';
import { describe, expect, it } from 'vitest';

import { flattenNodeUnion, getEmittableUnions } from '../src/unions';

const spec = getSpec();
const linkCategory = spec.categories.find(c => c.name === 'link')!;
const pdaSeedCategory = spec.categories.find(c => c.name === 'pdaSeed')!;

describe('getEmittableUnions', () => {
    it('returns the category-main union (the standalone twin of a `registered…`), sorted alphabetically', () => {
        expect(getEmittableUnions(linkCategory).map(u => u.name)).toEqual(['linkNode']);
        expect(getEmittableUnions(pdaSeedCategory).map(u => u.name)).toEqual(['pdaSeedNode']);
    });

    it('skips category-registry unions (`registered*`)', () => {
        expect(getEmittableUnions(linkCategory).map(u => u.name)).not.toContain('registeredLinkNode');
    });

    it('skips inline / synthetic unions that lack a `registered<Name>` twin (e.g. constantPdaSeedValue)', () => {
        // `constantPdaSeedValue` is the `constantPdaSeedNode.value` attribute's
        // union; it has no `registered<Name>` twin (it's not a category
        // dispatch enum) so the generator must not emit it as its own Rust
        // enum. A later PR with inline-union support will handle it.
        expect(getEmittableUnions(pdaSeedCategory).map(u => u.name)).not.toContain('constantPdaSeedValue');
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
