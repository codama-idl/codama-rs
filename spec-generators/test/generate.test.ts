import { getFromRenderMap } from '@codama/fragments';
import { getSpec } from '@codama/spec';
import { describe, expect, it } from 'vitest';

import { type GenerateOptions, getRenderMap, validateRenderOptions } from '../src/index';

function options(overrides: Partial<GenerateOptions> = {}): GenerateOptions {
    return {
        outputDir: '/tmp/unused',
        targetSpecMajor: 1,
        ...overrides,
    };
}

describe('validateRenderOptions', () => {
    const spec = getSpec();

    it('accepts the active v1 spec with defaulted options', () => {
        expect(() => validateRenderOptions(spec, options())).not.toThrow();
    });

    it('throws when targetSpecMajor does not match the spec version', () => {
        expect(() => validateRenderOptions(spec, options({ targetSpecMajor: 9 }))).toThrow(
            /targetSpecMajor=9.*major 1/,
        );
    });

    it('throws on a malformed spec version', () => {
        const broken = { ...spec, version: 'not-a-version' };
        expect(() => validateRenderOptions(broken, options())).toThrow(/unable to parse spec version "not-a-version"/);
    });
});

describe('getRenderMap', () => {
    const map = getRenderMap(getSpec(), options());

    it('emits one .rs file per node, one per emittable union, a per-category mod.rs, and a root mod.rs', () => {
        const keys = [...map.keys()].toSorted();
        expect(keys).toEqual([
            'count_nodes/count_node.rs',
            'count_nodes/fixed_count_node.rs',
            'count_nodes/mod.rs',
            'count_nodes/prefixed_count_node.rs',
            'count_nodes/remainder_count_node.rs',
            'discriminator_nodes/constant_discriminator_node.rs',
            'discriminator_nodes/discriminator_node.rs',
            'discriminator_nodes/field_discriminator_node.rs',
            'discriminator_nodes/mod.rs',
            'discriminator_nodes/size_discriminator_node.rs',
            'link_nodes/account_link_node.rs',
            'link_nodes/defined_type_link_node.rs',
            'link_nodes/instruction_account_link_node.rs',
            'link_nodes/instruction_argument_link_node.rs',
            'link_nodes/instruction_link_node.rs',
            'link_nodes/link_node.rs',
            'link_nodes/mod.rs',
            'link_nodes/pda_link_node.rs',
            'link_nodes/program_link_node.rs',
            'mod.rs',
        ]);
    });

    it('per-node pages resolve their crate imports to a grouped `use crate::{…}` block plus the macro line', () => {
        const entry = getFromRenderMap(map, 'link_nodes/account_link_node.rs');
        expect(entry.content).toContain('use crate::{CamelCaseString, HasName, ProgramLinkNode};');
        expect(entry.content).toContain('use codama_nodes_derive::node;');
    });

    it('per-node pages declare the struct, the From impl, and the HasName impl', () => {
        const entry = getFromRenderMap(map, 'link_nodes/account_link_node.rs');
        expect(entry.content).toContain('#[node]');
        expect(entry.content).toContain('pub struct AccountLinkNode {');
        expect(entry.content).toContain('impl From<AccountLinkNode> for crate::Node {');
        expect(entry.content).toContain('impl HasName for AccountLinkNode {');
    });

    it('emits the LinkNode union with the node_union macro, every variant, and a HasName impl', () => {
        const entry = getFromRenderMap(map, 'link_nodes/link_node.rs');
        expect(entry.content).toContain('use codama_nodes_derive::node_union;');
        expect(entry.content).toContain('#[node_union]');
        expect(entry.content).toContain('pub enum LinkNode {');
        expect(entry.content).toContain('Account(AccountLinkNode),');
        expect(entry.content).toContain('Program(ProgramLinkNode),');
        expect(entry.content).toContain('impl HasName for LinkNode {');
        expect(entry.content).toContain('LinkNode::Account(node) => node.name(),');
    });

    it('emits link_nodes/mod.rs listing every per-file module alphabetically with mod + pub use lines', () => {
        const entry = getFromRenderMap(map, 'link_nodes/mod.rs');
        expect(entry.content).toContain('mod account_link_node;');
        expect(entry.content).toContain('mod link_node;');
        expect(entry.content).toContain('mod program_link_node;');
        expect(entry.content).toContain('pub use account_link_node::*;');
        expect(entry.content).toContain('pub use link_node::*;');
    });

    it('emits a root mod.rs that re-exports every per-category subdirectory', () => {
        const entry = getFromRenderMap(map, 'mod.rs');
        for (const dir of ['count_nodes', 'discriminator_nodes', 'link_nodes']) {
            expect(entry.content).toContain(`mod ${dir};`);
            expect(entry.content).toContain(`pub use ${dir}::*;`);
        }
    });

    describe('count category', () => {
        it('routes FixedCountNode through Node::Count and emits the u64 field plus #[derive(Copy)]', () => {
            const entry = getFromRenderMap(map, 'count_nodes/fixed_count_node.rs');
            expect(entry.content).toContain('#[node]');
            expect(entry.content).toContain('#[derive(Copy)]');
            expect(entry.content).toContain('pub struct FixedCountNode {');
            expect(entry.content).toContain('pub value: u64,');
            expect(entry.content).toContain('impl From<FixedCountNode> for crate::Node {');
            expect(entry.content).toContain('crate::Node::Count(val.into())');
        });

        it('emits RemainderCountNode as an empty struct with #[derive(Copy, Default)]', () => {
            const entry = getFromRenderMap(map, 'count_nodes/remainder_count_node.rs');
            expect(entry.content).toContain('#[derive(Copy, Default)]');
            expect(entry.content).toContain('pub struct RemainderCountNode {}');
        });

        it('emits the CountNode union with Fixed/Prefixed/Remainder variants and no HasName impl', () => {
            const entry = getFromRenderMap(map, 'count_nodes/count_node.rs');
            expect(entry.content).toContain('pub enum CountNode {');
            expect(entry.content).toContain('Fixed(FixedCountNode),');
            expect(entry.content).toContain('Prefixed(PrefixedCountNode),');
            expect(entry.content).toContain('Remainder(RemainderCountNode),');
            expect(entry.content).not.toContain('impl HasName for CountNode');
        });
    });

    describe('discriminator category', () => {
        it('routes through Node::Discriminator and emits u64 offset/size fields', () => {
            const constant = getFromRenderMap(map, 'discriminator_nodes/constant_discriminator_node.rs');
            expect(constant.content).toContain('pub offset: u64,');
            expect(constant.content).toContain('crate::Node::Discriminator(val.into())');
            const size = getFromRenderMap(map, 'discriminator_nodes/size_discriminator_node.rs');
            expect(size.content).toContain('#[derive(Copy)]');
            expect(size.content).toContain('pub size: u64,');
        });

        it('emits HasName only on FieldDiscriminatorNode (only member with a stringIdentifier name)', () => {
            const field = getFromRenderMap(map, 'discriminator_nodes/field_discriminator_node.rs');
            expect(field.content).toContain('impl HasName for FieldDiscriminatorNode {');
            const constant = getFromRenderMap(map, 'discriminator_nodes/constant_discriminator_node.rs');
            expect(constant.content).not.toContain('impl HasName');
            const union = getFromRenderMap(map, 'discriminator_nodes/discriminator_node.rs');
            // Not every variant has a name -> no union HasName impl.
            expect(union.content).not.toContain('impl HasName for DiscriminatorNode');
        });
    });
});
