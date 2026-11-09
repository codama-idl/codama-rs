import { getSpec } from '@codama/spec';
import { describe, expect, it } from 'vitest';

import { getNodePageFragment } from '../../src/fragments/nodePage';

const spec = getSpec();
const nodeOf = (kind: string) => spec.categories.flatMap(c => c.nodes).find(n => n.kind === kind)!;

describe('getNodePageFragment', () => {
    it('composes the struct and HasIdentifier impl in order, separated by blank lines', () => {
        const result = getNodePageFragment(nodeOf('programLinkNode'), spec);
        const sections = result.content.split('\n\n');
        // [0] struct block, [1] HasIdentifier impl block. The `Node` conversions live in `node.rs`.
        expect(sections).toHaveLength(2);
        expect(sections[0]).toMatch(/#\[node\][\s\S]*pub struct ProgramLinkNode/);
        expect(sections[1]).toMatch(/^impl HasIdentifier for ProgramLinkNode/);
    });

    it('appends the HasTransforms impl after the HasIdentifier impl when the node has a transforms attribute', () => {
        const result = getNodePageFragment(nodeOf('definedTypeLinkNode'), spec);
        const sections = result.content.split('\n\n');
        expect(sections.at(-2)).toMatch(/^impl HasIdentifier for DefinedTypeLinkNode/);
        expect(sections.at(-1)).toMatch(/^impl HasTransforms for DefinedTypeLinkNode/);
        expect([...result.imports.keys()]).toContain('crate::HasTransforms');
    });

    it('omits the HasIdentifier impl when the node has no required identifier', () => {
        const out = getNodePageFragment(nodeOf('remainderCountNode'), spec).content;
        expect(out).not.toContain('HasIdentifier');
    });

    it('propagates every fragment’s crate-rooted imports through to the page fragment', () => {
        const result = getNodePageFragment(nodeOf('programLinkNode'), spec);
        const imports = [...result.imports.keys()].toSorted();
        // The struct's own name is not imported — it's defined in the same file.
        expect(imports).toEqual(['codama_nodes_derive::node', 'crate::HasIdentifier', 'crate::Identifier']);
    });
});
