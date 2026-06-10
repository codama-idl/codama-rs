import { type Spec } from '@codama/spec';
import { attribute, defineNode, stringIdentifier } from '@codama/spec/api';
import { describe, expect, it } from 'vitest';

import { getNodePageFragment } from '../../src/fragments/nodePage';

const linkRouting = { mode: 'wrapped' as const, nodeVariant: 'Link' };
const EMPTY_SPEC: Spec = { version: '1.0.0', categories: [] };

describe('getNodePageFragment', () => {
    it('composes the struct, From impl, and HasName impl in order, separated by blank lines', () => {
        const spec = defineNode('programLinkNode', {
            attributes: [attribute('name', stringIdentifier())],
        });
        const result = getNodePageFragment(spec, linkRouting, EMPTY_SPEC);
        const sections = result.content.split('\n\n');
        // [0] struct block, [1] From impl block, [2] HasName impl block.
        expect(sections[0]).toMatch(/#\[node\][\s\S]*pub struct ProgramLinkNode/);
        expect(sections[1]).toMatch(/^impl From<ProgramLinkNode>/);
        expect(sections[2]).toMatch(/^impl HasName for ProgramLinkNode/);
    });

    it('omits the HasName impl when the node has no name: stringIdentifier() attribute', () => {
        const spec = defineNode('emptyNode', { attributes: [] });
        const out = getNodePageFragment(spec, linkRouting, EMPTY_SPEC).content;
        expect(out).not.toContain('HasName');
    });

    it('propagates every fragment’s crate-rooted imports through to the page fragment', () => {
        const spec = defineNode('programLinkNode', {
            attributes: [attribute('name', stringIdentifier())],
        });
        const result = getNodePageFragment(spec, linkRouting, EMPTY_SPEC);
        const imports = [...result.imports.keys()].toSorted();
        // `crate::Node` is written absolutely inside the From impl, so
        // no import is emitted for it. The struct's own name is not
        // imported either — it's defined in the same file.
        expect(imports).toEqual(['codama_nodes_derive::node', 'crate::CamelCaseString', 'crate::HasName']);
    });
});
