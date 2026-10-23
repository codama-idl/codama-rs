import { getSpec } from '@codama/spec';
import { describe, expect, it } from 'vitest';

import { getUnionPageFragment } from '../../src/fragments/unionPage';

const spec = getSpec();
const linkCategory = spec.categories.find(c => c.name === 'link')!;
const linkUnion = linkCategory.unions.find(u => u.name === 'linkNode')!;
const pdaSeedCategory = spec.categories.find(c => c.name === 'pdaSeed')!;
const constantPdaSeedValueUnion = pdaSeedCategory.unions.find(u => u.name === 'constantPdaSeedValue')!;
const contextualValueCategory = spec.categories.find(c => c.name === 'contextualValue')!;
const pdaValuePdaUnion = contextualValueCategory.unions.find(u => u.name === 'pdaValuePda')!;

describe('getUnionPageFragment', () => {
    it('emits #[node_union] and a PascalCase enum name', () => {
        const result = getUnionPageFragment(linkUnion, spec);
        expect(result.content).toContain('#[node_union]');
        expect(result.content).toContain('pub enum LinkNode {');
    });

    it('lists every member of the flattened union as a variant, sorted alphabetically', () => {
        const result = getUnionPageFragment(linkUnion, spec);
        const variants = result.content.match(/^\s*(\w+)\(/gm)?.map(s => s.trim().replace('(', '')) ?? [];
        expect(variants).toEqual(['Account', 'DefinedType', 'Instruction', 'InstructionAccount', 'Pda', 'Program']);
    });

    it('emits an `impl HasIdentifier for PdaValuePda` block dispatching to each variant', () => {
        const result = getUnionPageFragment(pdaValuePdaUnion, spec);
        expect(result.content).toContain('impl HasIdentifier for PdaValuePda {');
        expect(result.content).toContain('PdaValuePda::Pda(node) => node.identifier(),');
        expect(result.content).toContain('PdaValuePda::PdaLink(node) => node.identifier(),');
    });

    it('emits an `impl HasIdentifier for LinkNode` because every link node has a required identifier', () => {
        const result = getUnionPageFragment(linkUnion, spec);
        expect(result.content).toContain('impl HasIdentifier for LinkNode {');
    });

    it('carries the codama_nodes_derive::node_union import plus the crate-rooted imports for every member type and the HasIdentifier helpers', () => {
        const result = getUnionPageFragment(pdaValuePdaUnion, spec);
        const imports = [...result.imports.keys()].toSorted();
        expect(imports).toContain('codama_nodes_derive::node_union');
        expect(imports).toContain('crate::PdaNode');
        expect(imports).toContain('crate::PdaLinkNode');
        expect(imports).toContain('crate::HasIdentifier');
        expect(imports).toContain('crate::CamelCaseString');
    });

    it('strips the longest common PascalCase suffix when naming variants of an inline union', () => {
        const result = getUnionPageFragment(constantPdaSeedValueUnion, spec);
        expect(result.content).toContain('pub enum ConstantPdaSeedValue {');
        expect(result.content).toContain('ProgramId(ProgramIdValueNode),');
        expect(result.content).toContain('Integer(IntegerValueNode),');
        expect(result.content).toContain('String(StringValueNode),');
        // Not every variant has a stringIdentifier name -> no HasIdentifier impl.
        expect(result.content).not.toContain('impl HasIdentifier for ConstantPdaSeedValue');
    });
});
