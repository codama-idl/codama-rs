import { getSpec } from '@codama/spec';
import { describe, expect, it } from 'vitest';

import { getUnionPageFragment } from '../../src/fragments/unionPage';

const spec = getSpec();
const linkCategory = spec.categories.find(c => c.name === 'link')!;
const linkUnion = linkCategory.unions.find(u => u.name === 'linkNode')!;

describe('getUnionPageFragment', () => {
    it('emits #[node_union] and a PascalCase enum name', () => {
        const result = getUnionPageFragment(linkUnion, spec);
        expect(result.content).toContain('#[node_union]');
        expect(result.content).toContain('pub enum LinkNode {');
    });

    it('lists every member of the flattened union as a variant, sorted alphabetically', () => {
        const result = getUnionPageFragment(linkUnion, spec);
        // The spec union `linkNode` references `union(\'registeredLinkNode\')`
        // which expands to 7 leaf nodes; the generator strips the
        // `LinkNode` suffix and pascalCases the rest.
        const variants = result.content.match(/^\s*(\w+)\(/gm)?.map(s => s.trim().replace('(', '')) ?? [];
        expect(variants).toEqual([
            'Account',
            'DefinedType',
            'Instruction',
            'InstructionAccount',
            'InstructionArgument',
            'Pda',
            'Program',
        ]);
    });

    it('emits an `impl HasName for LinkNode` block dispatching to each variant', () => {
        const result = getUnionPageFragment(linkUnion, spec);
        expect(result.content).toContain('impl HasName for LinkNode {');
        expect(result.content).toContain('LinkNode::Account(node) => node.name(),');
        expect(result.content).toContain('LinkNode::Program(node) => node.name(),');
    });

    it('carries the codama_nodes_derive::node_union import plus the crate-rooted imports for every member type and the HasName helpers', () => {
        const result = getUnionPageFragment(linkUnion, spec);
        const imports = [...result.imports.keys()].toSorted();
        expect(imports).toContain('codama_nodes_derive::node_union');
        expect(imports).toContain('crate::AccountLinkNode');
        expect(imports).toContain('crate::ProgramLinkNode');
        expect(imports).toContain('crate::HasName');
        expect(imports).toContain('crate::CamelCaseString');
    });
});
