import { getSpec } from '@codama/spec';
import { describe, expect, it } from 'vitest';

import { getNodePageFragment } from '../../src/fragments/nodePage';

const linkRouting = { mode: 'wrapped' as const, nodeVariant: 'Link' };
const spec = getSpec();
const nodeOf = (kind: string) => spec.categories.flatMap(c => c.nodes).find(n => n.kind === kind)!;

describe('getNodePageFragment', () => {
    it('composes the struct, From impl, and HasIdentifier impl in order, separated by blank lines', () => {
        const result = getNodePageFragment(nodeOf('programLinkNode'), linkRouting, spec);
        const sections = result.content.split('\n\n');
        // [0] struct block, [1] From impl block, [2] HasIdentifier impl block.
        expect(sections[0]).toMatch(/#\[node\][\s\S]*pub struct ProgramLinkNode/);
        expect(sections[1]).toMatch(/^impl From<ProgramLinkNode>/);
        expect(sections[2]).toMatch(/^impl HasIdentifier for ProgramLinkNode/);
    });

    it('omits the HasIdentifier impl when the node has no required identifier', () => {
        const out = getNodePageFragment(nodeOf('remainderCountNode'), linkRouting, spec).content;
        expect(out).not.toContain('HasIdentifier');
    });

    it('propagates every fragment’s crate-rooted imports through to the page fragment', () => {
        const result = getNodePageFragment(nodeOf('programLinkNode'), linkRouting, spec);
        const imports = [...result.imports.keys()].toSorted();
        // `crate::Node` is written absolutely inside the From impl, so
        // no import is emitted for it. The struct's own name is not
        // imported either — it's defined in the same file.
        expect(imports).toEqual(['codama_nodes_derive::node', 'crate::CamelCaseString', 'crate::HasIdentifier']);
    });
});
