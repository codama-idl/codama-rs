import { getFromRenderMap } from '@codama/fragments';
import { getSpec } from '@codama/spec';
import { describe, expect, it } from 'vitest';

import { type GenerateOptions, getRenderMap, validateRenderOptions } from '../src/index';

function options(overrides: Partial<GenerateOptions> = {}): GenerateOptions {
    return {
        outputDir: '/tmp/unused',
        targetSpecMajor: 2,
        ...overrides,
    };
}

describe('validateRenderOptions', () => {
    const spec = getSpec();

    it('accepts the active spec with defaulted options', () => {
        expect(() => validateRenderOptions(spec, options())).not.toThrow();
    });

    it('throws when targetSpecMajor does not match the spec version', () => {
        expect(() => validateRenderOptions(spec, options({ targetSpecMajor: 9 }))).toThrow(
            /targetSpecMajor=9.*major 2/,
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
            'account_node.rs',
            'codama_version.rs',
            'constant_node.rs',
            'contextual_value_nodes/account_bump_value_node.rs',
            'contextual_value_nodes/account_data_value_node.rs',
            'contextual_value_nodes/account_value_node.rs',
            'contextual_value_nodes/conditional_value_condition.rs',
            'contextual_value_nodes/conditional_value_node.rs',
            'contextual_value_nodes/contextual_value_node.rs',
            'contextual_value_nodes/data_value_node.rs',
            'contextual_value_nodes/identity_value_node.rs',
            'contextual_value_nodes/instruction_input_value_node.rs',
            'contextual_value_nodes/mod.rs',
            'contextual_value_nodes/payer_value_node.rs',
            'contextual_value_nodes/pda_seed_value_node.rs',
            'contextual_value_nodes/pda_seed_value_value.rs',
            'contextual_value_nodes/pda_value_node.rs',
            'contextual_value_nodes/pda_value_pda.rs',
            'contextual_value_nodes/pda_value_program_id.rs',
            'contextual_value_nodes/program_id_value_node.rs',
            'count_nodes/count_node.rs',
            'count_nodes/fixed_count_node.rs',
            'count_nodes/mod.rs',
            'count_nodes/prefixed_count_node.rs',
            'count_nodes/remainder_count_node.rs',
            'count_nodes/sentinel_count_node.rs',
            'defined_type_node.rs',
            'discriminator_nodes/constant_discriminator_node.rs',
            'discriminator_nodes/discriminator_node.rs',
            'discriminator_nodes/field_discriminator_node.rs',
            'discriminator_nodes/mod.rs',
            'discriminator_nodes/size_discriminator_node.rs',
            'display_nodes/amount_number_display_node.rs',
            'display_nodes/display_node.rs',
            'display_nodes/enum_variant_display_node.rs',
            'display_nodes/instruction_account_display_node.rs',
            'display_nodes/instruction_display_node.rs',
            'display_nodes/mod.rs',
            'display_nodes/number_display_node.rs',
            'display_nodes/string_display_node.rs',
            'display_nodes/struct_field_display_node.rs',
            'display_nodes/unit_number_display_node.rs',
            'display_nodes/value_label_display_node.rs',
            'error_node.rs',
            'event_node.rs',
            'instruction_account_node.rs',
            'instruction_byte_delta_node.rs',
            'instruction_byte_delta_value.rs',
            'instruction_node.rs',
            'instruction_remaining_accounts_node.rs',
            'instruction_status_node.rs',
            'link_nodes/account_link_node.rs',
            'link_nodes/defined_type_link_node.rs',
            'link_nodes/instruction_account_link_node.rs',
            'link_nodes/instruction_link_node.rs',
            'link_nodes/link_node.rs',
            'link_nodes/mod.rs',
            'link_nodes/pda_link_node.rs',
            'link_nodes/program_link_node.rs',
            'mod.rs',
            'pda_node.rs',
            'pda_seed_nodes/constant_pda_seed_node.rs',
            'pda_seed_nodes/constant_pda_seed_value.rs',
            'pda_seed_nodes/mod.rs',
            'pda_seed_nodes/pda_seed_node.rs',
            'pda_seed_nodes/variable_pda_seed_node.rs',
            'plugin_node.rs',
            'program_node.rs',
            'provided_node.rs',
            'root_node.rs',
            'shared/base.rs',
            'shared/bytes_encoding.rs',
            'shared/default_value_strategy.rs',
            'shared/display_skip.rs',
            'shared/endianness.rs',
            'shared/float_format.rs',
            'shared/instruction_lifecycle.rs',
            'shared/integer_format.rs',
            'shared/is_signer.rs',
            'shared/mod.rs',
            'shared/optional_account_strategy.rs',
            'shared/post_offset_strategy.rs',
            'shared/pre_offset_strategy.rs',
            'shared/sentinel_count_strategy.rs',
            'text_node.rs',
            'transform_nodes/fixed_size_transform_node.rs',
            'transform_nodes/hidden_prefix_transform_node.rs',
            'transform_nodes/hidden_suffix_transform_node.rs',
            'transform_nodes/mod.rs',
            'transform_nodes/post_offset_transform_node.rs',
            'transform_nodes/pre_offset_transform_node.rs',
            'transform_nodes/sentinel_transform_node.rs',
            'transform_nodes/size_prefix_transform_node.rs',
            'transform_nodes/transform_node.rs',
            'type_nodes/array_type_node.rs',
            'type_nodes/boolean_type_node.rs',
            'type_nodes/bytes_type_node.rs',
            'type_nodes/date_time_type_node.rs',
            'type_nodes/duration_type_node.rs',
            'type_nodes/enum_type_node.rs',
            'type_nodes/enum_variant_type_node.rs',
            'type_nodes/fixed_point_type_node.rs',
            'type_nodes/float_type_node.rs',
            'type_nodes/integer_type_node.rs',
            'type_nodes/map_type_node.rs',
            'type_nodes/mod.rs',
            'type_nodes/option_type_node.rs',
            'type_nodes/public_key_type_node.rs',
            'type_nodes/remainder_option_type_node.rs',
            'type_nodes/set_type_node.rs',
            'type_nodes/string_type_node.rs',
            'type_nodes/struct_field_type_node.rs',
            'type_nodes/struct_type_node.rs',
            'type_nodes/tuple_type_node.rs',
            'type_nodes/zeroable_option_type_node.rs',
            'value_nodes/array_value_node.rs',
            'value_nodes/boolean_value_node.rs',
            'value_nodes/bytes_value_node.rs',
            'value_nodes/constant_value_node.rs',
            'value_nodes/enum_value_node.rs',
            'value_nodes/float_value_node.rs',
            'value_nodes/injectable_integer_value_node.rs',
            'value_nodes/injectable_unit_value_node.rs',
            'value_nodes/injected_value_node.rs',
            'value_nodes/integer_value_node.rs',
            'value_nodes/map_entry_value_node.rs',
            'value_nodes/map_value_node.rs',
            'value_nodes/mod.rs',
            'value_nodes/none_value_node.rs',
            'value_nodes/public_key_value_node.rs',
            'value_nodes/set_value_node.rs',
            'value_nodes/some_value_node.rs',
            'value_nodes/string_value_node.rs',
            'value_nodes/struct_field_value_node.rs',
            'value_nodes/struct_value_node.rs',
            'value_nodes/tuple_value_node.rs',
            'value_nodes/value_node.rs',
        ]);
    });

    it('emits the `value` category union via the RegisteredNodes derive (standalone + #[registered] split)', () => {
        // Render-map content is pre-format (no indentation); `cargo fmt`
        // restores it on disk.
        const entry = getFromRenderMap(map, 'value_nodes/value_node.rs');
        expect(entry.content).toContain('use codama_nodes_derive::{node_union, RegisteredNodes};');
        expect(entry.content).toContain('#[derive(RegisteredNodes)]');
        expect(entry.content).toContain('#[node_union]');
        expect(entry.content).toContain('pub enum RegisteredValueNode {');
        // Standalone variants (alphabetical).
        expect(entry.content).toContain('Array(ArrayValueNode),');
        expect(entry.content).toContain('Tuple(TupleValueNode),');
        // `#[registered]`-only variants follow.
        expect(entry.content).toContain('#[registered]\nMapEntry(MapEntryValueNode),');
        expect(entry.content).toContain('#[registered]\nStructField(StructFieldValueNode),');
    });

    it('per-node pages resolve their crate imports to a grouped `use crate::{…}` block plus the macro line', () => {
        const entry = getFromRenderMap(map, 'link_nodes/account_link_node.rs');
        expect(entry.content).toContain('use crate::{CamelCaseString, HasIdentifier, PluginNode, ProgramLinkNode};');
        expect(entry.content).toContain('use codama_nodes_derive::node;');
    });

    it('per-node pages declare the struct, the From impl, and the HasIdentifier impl', () => {
        const entry = getFromRenderMap(map, 'link_nodes/account_link_node.rs');
        expect(entry.content).toContain('#[node]');
        expect(entry.content).toContain('pub struct AccountLinkNode {');
        expect(entry.content).toContain('// Data.\npub identifier: CamelCaseString,');
        expect(entry.content).toContain('impl From<AccountLinkNode> for crate::Node {');
        expect(entry.content).toContain('impl HasIdentifier for AccountLinkNode {');
    });

    it('emits the LinkNode union with the node_union macro, every variant, and a HasIdentifier impl', () => {
        const entry = getFromRenderMap(map, 'link_nodes/link_node.rs');
        expect(entry.content).toContain('use codama_nodes_derive::node_union;');
        expect(entry.content).toContain('#[node_union]');
        expect(entry.content).toContain('pub enum LinkNode {');
        expect(entry.content).toContain('Account(AccountLinkNode),');
        expect(entry.content).toContain('Program(ProgramLinkNode),');
        expect(entry.content).toContain('impl HasIdentifier for LinkNode {');
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
        for (const dir of [
            'contextual_value_nodes',
            'count_nodes',
            'discriminator_nodes',
            'display_nodes',
            'link_nodes',
            'pda_seed_nodes',
            'shared',
            'transform_nodes',
            'type_nodes',
            'value_nodes',
        ]) {
            expect(entry.content).toContain(`mod ${dir};`);
            expect(entry.content).toContain(`pub use ${dir}::*;`);
        }
    });

    it('renders the base `plugins` attribute as the last field of every node page', () => {
        const pages = [...map.values()].filter(f => /^#\[(node|type_node)\]$/m.test(f.content));
        expect(pages).toHaveLength(94);
        for (const page of pages) {
            expect(page.content).toMatch(
                /\/\/ Children\.\n[^}]*#\[serde\(default, skip_serializing_if = "crate::is_default"\)\]\npub plugins: Vec<PluginNode>,\n}/,
            );
        }
    });

    it('emits HasIdentifier for the 24 nodes with a required identifier and the 2 unions over them', () => {
        const impls = [...map.entries()]
            .filter(([, f]) => f.content.includes('impl HasIdentifier for'))
            .map(([k]) => k);
        expect(impls).toHaveLength(26);
        expect(impls).toContain('link_nodes/link_node.rs');
        expect(impls).toContain('contextual_value_nodes/pda_value_pda.rs');
        expect(impls).not.toContain('value_nodes/public_key_value_node.rs');
    });

    it('emits HasTransforms for the 19 nodes with a transforms attribute and for no union', () => {
        const impls = [...map.entries()]
            .filter(([, f]) => f.content.includes('impl HasTransforms for'))
            .map(([k]) => k);
        expect(impls).toHaveLength(19);
        expect(impls).toContain('type_nodes/integer_type_node.rs');
        expect(impls).not.toContain('type_nodes/struct_field_type_node.rs');
        expect(impls.filter(k => !k.startsWith('type_nodes/'))).toEqual(['link_nodes/defined_type_link_node.rs']);
    });

    it('emits both the HasIdentifier and the HasTransforms impl for DefinedTypeLinkNode', () => {
        const entry = getFromRenderMap(map, 'link_nodes/defined_type_link_node.rs');
        expect(entry.content).toContain(
            'use crate::{CamelCaseString, HasIdentifier, HasTransforms, PluginNode, ProgramLinkNode, TransformNode};',
        );
        expect(entry.content).toContain('impl HasIdentifier for DefinedTypeLinkNode {');
        expect(entry.content).toContain('impl HasTransforms for DefinedTypeLinkNode {');
    });

    describe('count category', () => {
        it('routes FixedCountNode through Node::Count and emits the u64 field plus #[derive(Default)]', () => {
            const entry = getFromRenderMap(map, 'count_nodes/fixed_count_node.rs');
            expect(entry.content).toContain('#[node]');
            expect(entry.content).toContain('#[derive(Default)]');
            expect(entry.content).toContain('pub struct FixedCountNode {');
            expect(entry.content).toContain('pub value: u64,');
            expect(entry.content).toContain('impl From<FixedCountNode> for crate::Node {');
            expect(entry.content).toContain('crate::Node::Count(val.into())');
        });

        it('emits RemainderCountNode with the base `plugins` field only and #[derive(Default)]', () => {
            const entry = getFromRenderMap(map, 'count_nodes/remainder_count_node.rs');
            expect(entry.content).toContain('#[derive(Default)]');
            expect(entry.content).toContain('pub struct RemainderCountNode {\n// Children.\n');
            expect(entry.content).not.toContain('// Data.');
        });

        it('emits the CountNode union with Fixed/Prefixed/Remainder/Sentinel variants and no HasIdentifier impl', () => {
            const entry = getFromRenderMap(map, 'count_nodes/count_node.rs');
            expect(entry.content).toContain('pub enum CountNode {');
            expect(entry.content).toContain('Fixed(FixedCountNode),');
            expect(entry.content).toContain('Prefixed(PrefixedCountNode),');
            expect(entry.content).toContain('Remainder(RemainderCountNode),');
            expect(entry.content).toContain('Sentinel(SentinelCountNode),');
            expect(entry.content).not.toContain('impl HasIdentifier for CountNode');
        });
    });

    describe('discriminator category', () => {
        it('routes through Node::Discriminator and emits u64 offset/size fields', () => {
            const constant = getFromRenderMap(map, 'discriminator_nodes/constant_discriminator_node.rs');
            expect(constant.content).toContain('pub offset: u64,');
            expect(constant.content).toContain('crate::Node::Discriminator(val.into())');
            const size = getFromRenderMap(map, 'discriminator_nodes/size_discriminator_node.rs');
            expect(size.content).toContain('#[derive(Default)]');
            expect(size.content).toContain('pub size: u64,');
        });

        it('renders FieldDiscriminatorNode.path as `Path` without Default and emits no HasIdentifier impl', () => {
            const field = getFromRenderMap(map, 'discriminator_nodes/field_discriminator_node.rs');
            expect(field.content).toContain('use crate::{Path, PluginNode};');
            expect(field.content).toContain('pub path: Path,');
            expect(field.content).not.toContain('Default');
            expect(field.content).not.toContain('impl HasIdentifier');
            const constant = getFromRenderMap(map, 'discriminator_nodes/constant_discriminator_node.rs');
            expect(constant.content).not.toContain('impl HasIdentifier');
            const union = getFromRenderMap(map, 'discriminator_nodes/discriminator_node.rs');
            expect(union.content).not.toContain('impl HasIdentifier for DiscriminatorNode');
        });
    });

    describe('display category', () => {
        it('routes display nodes through Node::Display', () => {
            const entry = getFromRenderMap(map, 'display_nodes/string_display_node.rs');
            expect(entry.content).toContain('pub struct StringDisplayNode {');
            expect(entry.content).toContain('crate::Node::Display(val.into())');
        });

        it('emits the flattened DisplayNode union over every display node (no registered twin)', () => {
            const entry = getFromRenderMap(map, 'display_nodes/display_node.rs');
            expect(entry.content).toContain('#[node_union]');
            expect(entry.content).toContain('pub enum DisplayNode {');
            expect(entry.content).toContain('AmountNumber(AmountNumberDisplayNode),');
            expect(entry.content).toContain('StructField(StructFieldDisplayNode),');
            // No `Registered…` enum: the twin has no registered-only members.
            expect(entry.content).not.toContain('RegisteredDisplayNode');
        });

        it('derives Default for scalar-only display nodes', () => {
            const entry = getFromRenderMap(map, 'display_nodes/string_display_node.rs');
            expect(entry.content).toContain('#[derive(Default)]');
        });

        it('boxes the injectable-union fields of AmountNumberDisplayNode', () => {
            const entry = getFromRenderMap(map, 'display_nodes/amount_number_display_node.rs');
            expect(entry.content).toContain('pub decimals: Box<InjectableIntegerValueNode>,');
            expect(entry.content).toContain('pub unit: Box<Option<InjectableUnitValueNode>>,');
        });
    });

    describe('pdaSeed category', () => {
        it('routes through Node::PdaSeed and references the inline ConstantPdaSeedValue enum', () => {
            const entry = getFromRenderMap(map, 'pda_seed_nodes/constant_pda_seed_node.rs');
            expect(entry.content).toContain('pub struct ConstantPdaSeedNode {');
            // Box rule: direct union fields are boxed.
            expect(entry.content).toContain('pub r#type: Box<TypeNode>,');
            // The value field type is the generated inline union, NOT the
            // pre-existing ValueNode.
            expect(entry.content).toContain('pub value: Box<ConstantPdaSeedValue>,');
            expect(entry.content).toContain('crate::Node::PdaSeed(val.into())');
        });

        it('emits the ConstantPdaSeedValue inline-union enum with stripped variant names', () => {
            const entry = getFromRenderMap(map, 'pda_seed_nodes/constant_pda_seed_value.rs');
            expect(entry.content).toContain('#[node_union]');
            expect(entry.content).toContain('pub enum ConstantPdaSeedValue {');
            expect(entry.content).toContain('ProgramId(ProgramIdValueNode),');
            // Sample of value-node variants (suffix `ValueNode` stripped).
            expect(entry.content).toContain('Integer(IntegerValueNode),');
            expect(entry.content).toContain('String(StringValueNode),');
            expect(entry.content).toContain('Boolean(BooleanValueNode),');
            expect(entry.content).not.toContain('impl HasIdentifier for ConstantPdaSeedValue');
        });

        it('routes VariablePdaSeedNode through Node::PdaSeed with HasIdentifier + a bare Docs field', () => {
            const entry = getFromRenderMap(map, 'pda_seed_nodes/variable_pda_seed_node.rs');
            expect(entry.content).toContain('pub struct VariablePdaSeedNode {');
            expect(entry.content).toContain('pub identifier: CamelCaseString,');
            expect(entry.content).toContain('pub docs: Docs,');
            // Box rule: direct union fields are boxed.
            expect(entry.content).toContain('pub r#type: Box<TypeNode>,');
            expect(entry.content).toContain('impl HasIdentifier for VariablePdaSeedNode {');
        });

        it('emits the PdaSeedNode category union with Constant/Variable variants and no HasIdentifier', () => {
            const entry = getFromRenderMap(map, 'pda_seed_nodes/pda_seed_node.rs');
            expect(entry.content).toContain('pub enum PdaSeedNode {');
            expect(entry.content).toContain('Constant(ConstantPdaSeedNode),');
            expect(entry.content).toContain('Variable(VariablePdaSeedNode),');
            expect(entry.content).not.toContain('impl HasIdentifier for PdaSeedNode');
        });
    });

    describe('value category', () => {
        it('routes every value node through Node::Value', () => {
            const entry = getFromRenderMap(map, 'value_nodes/boolean_value_node.rs');
            expect(entry.content).toContain('crate::Node::Value(val.into())');
        });

        it('applies the box-all-union rule on direct union fields (required → Box<T>)', () => {
            const constant = getFromRenderMap(map, 'value_nodes/constant_value_node.rs');
            expect(constant.content).toContain('pub r#type: Box<TypeNode>,');
            expect(constant.content).toContain('pub value: Box<ValueNode>,');
            const some = getFromRenderMap(map, 'value_nodes/some_value_node.rs');
            expect(some.content).toContain('pub value: Box<ValueNode>,');
            const entry = getFromRenderMap(map, 'value_nodes/map_entry_value_node.rs');
            expect(entry.content).toContain('pub key: Box<ValueNode>,');
            expect(entry.content).toContain('pub value: Box<ValueNode>,');
        });

        it('applies the box-all-union rule on optional union fields (Box<Option<T>>)', () => {
            const entry = getFromRenderMap(map, 'value_nodes/enum_value_node.rs');
            expect(entry.content).toContain('pub value: Box<Option<ValueNode>>,');
        });

        it('does NOT box Vec<union> fields (arrayValueNode / setValueNode / tupleValueNode)', () => {
            for (const file of ['array_value_node.rs', 'set_value_node.rs', 'tuple_value_node.rs']) {
                const entry = getFromRenderMap(map, `value_nodes/${file}`);
                expect(entry.content).toContain('pub items: Vec<ValueNode>,');
            }
        });

        it('renders integerValueNode.value as `Integer` and floatValueNode.value as `Decimal`, both without Default', () => {
            const integer = getFromRenderMap(map, 'value_nodes/integer_value_node.rs');
            expect(integer.content).toContain('pub value: Integer,');
            expect(integer.content).not.toContain('Default');
            const float = getFromRenderMap(map, 'value_nodes/float_value_node.rs');
            expect(float.content).toContain('use crate::{Decimal, PluginNode};');
            expect(float.content).toContain('pub value: Decimal,');
            expect(float.content).not.toContain('Default');
        });

        it('emits Default for noneValueNode (base attributes only)', () => {
            const entry = getFromRenderMap(map, 'value_nodes/none_value_node.rs');
            expect(entry.content).toContain('#[derive(Default)]');
            expect(entry.content).toContain('pub struct NoneValueNode {\n// Children.\n');
        });

        it('emits the InjectableIntegerValueNode inline-union enum with Injected/Integer variants', () => {
            const entry = getFromRenderMap(map, 'value_nodes/injectable_integer_value_node.rs');
            expect(entry.content).toContain('pub enum InjectableIntegerValueNode {');
            expect(entry.content).toContain('Injected(InjectedValueNode),');
            expect(entry.content).toContain('Integer(IntegerValueNode),');
        });
    });

    describe('contextualValue category', () => {
        it('routes every contextualValue node through Node::ContextualValue', () => {
            const entry = getFromRenderMap(map, 'contextual_value_nodes/account_value_node.rs');
            expect(entry.content).toContain('crate::Node::ContextualValue(val.into())');
        });

        it('emits the RegisteredContextualValueNode union via the RegisteredNodes derive', () => {
            const entry = getFromRenderMap(map, 'contextual_value_nodes/contextual_value_node.rs');
            expect(entry.content).toContain('#[derive(RegisteredNodes)]');
            expect(entry.content).toContain('pub enum RegisteredContextualValueNode {');
            // Standalone variants (alphabetical, stripped ValueNode).
            expect(entry.content).toContain('Account(AccountValueNode),');
            expect(entry.content).toContain('AccountBump(AccountBumpValueNode),');
            expect(entry.content).toContain('Data(DataValueNode),');
            // `#[registered]`-only: `pdaSeedValueNode`.
            expect(entry.content).toContain('#[registered]\nPdaSeed(PdaSeedValueNode),');
        });

        it('renders dataValueNode.path as `Path` and accountDataValueNode.path as `Option<Path>`', () => {
            const data = getFromRenderMap(map, 'contextual_value_nodes/data_value_node.rs');
            expect(data.content).toContain('pub path: Path,');
            expect(data.content).not.toContain('Default');
            const accountData = getFromRenderMap(map, 'contextual_value_nodes/account_data_value_node.rs');
            expect(accountData.content).toContain('pub path: Option<Path>,');
            expect(accountData.content).toContain('#[derive(Default)]');
        });

        it('emits the PdaValuePda inline-union with derived variant names (PdaLink, Pda)', () => {
            // Common suffix across [PdaLinkNode, PdaNode] is just `Node`,
            // so variants are `PdaLink` and `Pda` — not the old hand-written
            // `Linked` / `Nested`.
            const entry = getFromRenderMap(map, 'contextual_value_nodes/pda_value_pda.rs');
            expect(entry.content).toContain('pub enum PdaValuePda {');
            expect(entry.content).toContain('Pda(PdaNode),');
            expect(entry.content).toContain('PdaLink(PdaLinkNode),');
        });

        it('emits InstructionInputValueNode variants with the Value suffix (common suffix = Node)', () => {
            const entry = getFromRenderMap(map, 'contextual_value_nodes/instruction_input_value_node.rs');
            expect(entry.content).toContain('pub enum InstructionInputValueNode {');
            expect(entry.content).toContain('AccountValue(AccountValueNode),');
            expect(entry.content).toContain('DataValue(DataValueNode),');
            expect(entry.content).toContain('IntegerValue(IntegerValueNode),');
            expect(entry.content).toContain('ProgramLink(ProgramLinkNode),');
        });

        it('applies the box-all-union rule + Box<Option<T>> shapes on ConditionalValueNode', () => {
            const entry = getFromRenderMap(map, 'contextual_value_nodes/conditional_value_node.rs');
            expect(entry.content).toContain('pub condition: Box<ConditionalValueCondition>,');
            expect(entry.content).toContain('pub value: Box<Option<ValueNode>>,');
            expect(entry.content).toContain('pub if_true: Box<Option<InstructionInputValueNode>>,');
            expect(entry.content).toContain('pub if_false: Box<Option<InstructionInputValueNode>>,');
        });
    });

    describe('topLevel category (direct routing)', () => {
        it('emits no `impl From<…> for crate::Node` for any topLevel node', () => {
            for (const file of [
                'account_node.rs',
                'constant_node.rs',
                'defined_type_node.rs',
                'error_node.rs',
                'event_node.rs',
                'instruction_account_node.rs',
                'instruction_byte_delta_node.rs',
                'instruction_node.rs',
                'instruction_remaining_accounts_node.rs',
                'instruction_status_node.rs',
                'pda_node.rs',
                'program_node.rs',
                'provided_node.rs',
                'root_node.rs',
                'text_node.rs',
            ]) {
                const entry = getFromRenderMap(map, file);
                expect(entry.content).not.toContain('impl From<');
                expect(entry.content).not.toContain('crate::Node::');
            }
        });

        it('renders providedNode.node (an anyNode child) as a boxed `Box<Node>`', () => {
            const entry = getFromRenderMap(map, 'provided_node.rs');
            expect(entry.content).toContain('pub struct ProvidedNode {');
            expect(entry.content).toContain('pub node: Box<Node>,');
            expect(entry.content).toContain('impl HasIdentifier for ProvidedNode {');
        });

        it('renders instructionNode.provides as a bare `Vec<ProvidedNode>` and its optional display child', () => {
            const entry = getFromRenderMap(map, 'instruction_node.rs');
            expect(entry.content).toContain('pub provides: Vec<ProvidedNode>,');
            expect(entry.content).toContain('pub display: Option<InstructionDisplayNode>,');
        });

        it('emits instructionNode.discriminators as a bare `Vec` (matches the optional-array convention)', () => {
            const entry = getFromRenderMap(map, 'instruction_node.rs');
            expect(entry.content).toContain('pub discriminators: Vec<DiscriminatorNode>,');
            expect(entry.content).not.toContain('Option<Vec<DiscriminatorNode>>');
        });

        it('renders instructionAccountNode.accountLink as an optional `Option<AccountLinkNode>`', () => {
            const entry = getFromRenderMap(map, 'instruction_account_node.rs');
            expect(entry.content).toContain('pub account_link: Option<AccountLinkNode>,');
        });

        it('renders rootNode.standard as `pub standard: String`', () => {
            const entry = getFromRenderMap(map, 'root_node.rs');
            expect(entry.content).toContain('pub struct RootNode {');
            expect(entry.content).toContain('pub standard: String,');
            expect(entry.content).not.toContain('"codama"');
        });

        it('renders errorNode.code as `u32` and accountNode.size as `Option<u64>`', () => {
            expect(getFromRenderMap(map, 'error_node.rs').content).toContain('pub code: u32,');
            expect(getFromRenderMap(map, 'account_node.rs').content).toContain('pub size: Option<u64>,');
        });

        it('renders errorNode.message as `Text` and pluginNode.namespace as `Namespace`', () => {
            expect(getFromRenderMap(map, 'error_node.rs').content).toContain('pub message: Text,');
            expect(getFromRenderMap(map, 'plugin_node.rs').content).toContain('pub namespace: Namespace,');
        });

        it('renders optional scalars as `Option<T>` uniformly', () => {
            expect(getFromRenderMap(map, 'instruction_account_node.rs').content).toContain(
                'pub is_optional: Option<bool>,',
            );
            const remaining = getFromRenderMap(map, 'instruction_remaining_accounts_node.rs');
            expect(remaining.content).toContain('pub is_optional: Option<bool>,');
            expect(remaining.content).toContain('pub is_signer: Option<IsSigner>,');
            expect(remaining.content).toContain('pub is_writable: Option<bool>,');
            expect(getFromRenderMap(map, 'instruction_byte_delta_node.rs').content).toContain(
                'pub subtract: Option<bool>,',
            );
            expect(getFromRenderMap(map, 'instruction_status_node.rs').content).toContain('pub message: Option<Text>,');
            expect(getFromRenderMap(map, 'instruction_node.rs').content).toContain(
                'pub optional_account_strategy: Option<OptionalAccountStrategy>,',
            );
        });

        it('boxes every direct (non-Vec) union field', () => {
            expect(getFromRenderMap(map, 'constant_node.rs').content).toContain('pub r#type: Box<TypeNode>,');
            expect(getFromRenderMap(map, 'constant_node.rs').content).toContain('pub value: Box<ValueNode>,');
            expect(getFromRenderMap(map, 'event_node.rs').content).toContain('pub data: Box<TypeNode>,');
            expect(getFromRenderMap(map, 'instruction_byte_delta_node.rs').content).toContain(
                'pub value: Box<InstructionByteDeltaValue>,',
            );
            expect(getFromRenderMap(map, 'instruction_account_node.rs').content).toContain(
                'pub default_value: Box<Option<InstructionInputValueNode>>,',
            );
            expect(getFromRenderMap(map, 'account_node.rs').content).toContain('pub data: Box<TypeNode>,');
        });

        it('derives InstructionByteDeltaValue variant names from the longest common suffix (`Node`)', () => {
            const entry = getFromRenderMap(map, 'instruction_byte_delta_value.rs');
            expect(entry.content).toContain('pub enum InstructionByteDeltaValue {');
            expect(entry.content).toContain('AccountLink(AccountLinkNode),');
            expect(entry.content).toContain('DataValue(DataValueNode),');
            expect(entry.content).toContain('IntegerValue(IntegerValueNode),');
        });

        it('derives Default for all-Default-able nodes; suppresses it for nodes with opaque required fields', () => {
            expect(getFromRenderMap(map, 'program_node.rs').content).toContain('#[derive(Default)]');
            expect(getFromRenderMap(map, 'instruction_node.rs').content).toContain('#[derive(Default)]');
            expect(getFromRenderMap(map, 'pda_node.rs').content).toContain('#[derive(Default)]');
            expect(getFromRenderMap(map, 'error_node.rs').content).not.toContain('Default');
            expect(getFromRenderMap(map, 'instruction_status_node.rs').content).not.toContain('Default');
            expect(getFromRenderMap(map, 'root_node.rs').content).not.toContain('Default');
            expect(getFromRenderMap(map, 'account_node.rs').content).not.toContain('Default');
        });

        it('emits a `CODAMA_VERSION` constant pinned to the spec version without its pre-release part', () => {
            const entry = getFromRenderMap(map, 'codama_version.rs');
            expect(entry.content).toContain('pub const CODAMA_VERSION: &str = "2.0.0";');
        });
    });

    describe('shared literalUnion shells', () => {
        it('emits a generated `pub enum IsSigner` shell with the non-serde derive set only', () => {
            const entry = getFromRenderMap(map, 'shared/is_signer.rs');
            expect(entry.content).toContain('#[derive(Debug, PartialEq, Eq, Clone, Copy)]');
            expect(entry.content).toContain('pub enum IsSigner {');
            // Variants from the spec's `[true, false, "either"]` value-set,
            // in declaration order.
            expect(entry.content).toContain('True,');
            expect(entry.content).toContain('False,');
            expect(entry.content).toContain('Either,');
            // No serde / default machinery in the generated shell — bespoke
            // bool-or-string serde + Default live in the hand-written
            // companion file.
            expect(entry.content).not.toContain('Serialize');
            expect(entry.content).not.toContain('Deserialize');
            expect(entry.content).not.toContain('Default');
            expect(entry.content).not.toContain('rename_all');
        });

        it('emits `pub enum Base` with the numeric variants `Base2` and `Base10`', () => {
            const entry = getFromRenderMap(map, 'shared/base.rs');
            expect(entry.content).toContain('pub enum Base {\nBase2,\nBase10,\n}');
            expect(getFromRenderMap(map, 'type_nodes/fixed_point_type_node.rs').content).toContain(
                'pub base: Option<Base>,',
            );
        });
    });

    describe('transform category', () => {
        it('emits the 7 transform nodes and the TransformNode union into transform_nodes/', () => {
            expect([...map.keys()].filter(k => k.startsWith('transform_nodes/'))).toHaveLength(9);
            const entry = getFromRenderMap(map, 'transform_nodes/fixed_size_transform_node.rs');
            expect(entry.content).toContain('#[node]');
            expect(entry.content).toContain('crate::Node::Transform(val.into())');
        });

        it('emits the TransformNode union with suffix-stripped variants', () => {
            const entry = getFromRenderMap(map, 'transform_nodes/transform_node.rs');
            expect(entry.content).toContain('pub enum TransformNode {');
            expect(entry.content).toContain('FixedSize(FixedSizeTransformNode),');
            expect(entry.content).toContain('SizePrefix(SizePrefixTransformNode),');
        });
    });

    describe('type category', () => {
        it('emits the 18 standalone type nodes with `#[type_node]` and routes through `Node::Type`', () => {
            const pages = [...map.values()].filter(f => f.content.includes('#[type_node]'));
            expect(pages).toHaveLength(18);
            const entry = getFromRenderMap(map, 'type_nodes/integer_type_node.rs');
            expect(entry.content).toContain('#[type_node]');
            expect(entry.content).toContain('pub struct IntegerTypeNode {');
            expect(entry.content).toContain('crate::Node::Type(val.into())');
        });

        it('emits `enumVariantTypeNode` and `structFieldTypeNode` with `#[node]` (not `#[type_node]`)', () => {
            const variant = getFromRenderMap(map, 'type_nodes/enum_variant_type_node.rs');
            expect(variant.content).toContain('#[node]');
            expect(variant.content).not.toContain('#[type_node]');
            const structField = getFromRenderMap(map, 'type_nodes/struct_field_type_node.rs');
            expect(structField.content).toContain('#[node]');
            expect(structField.content).not.toContain('#[type_node]');
        });

        it('skips ALL type-category unions (TypeNode, RegisteredTypeNode, StandaloneTypeNode)', () => {
            for (const file of [
                'type_nodes/type_node.rs',
                'type_nodes/registered_type_node.rs',
                'type_nodes/standalone_type_node.rs',
            ]) {
                expect(map.get(file)).toBeUndefined();
            }
        });

        it('aligns generated widths with the spec (scale -> u32; enum-variant discriminator -> u32; option.fixed -> Option<bool>)', () => {
            expect(getFromRenderMap(map, 'type_nodes/fixed_point_type_node.rs').content).toContain('pub scale: u32,');
            expect(getFromRenderMap(map, 'type_nodes/enum_variant_type_node.rs').content).toContain(
                'pub discriminator: Option<u32>,',
            );
            expect(getFromRenderMap(map, 'type_nodes/option_type_node.rs').content).toContain(
                'pub fixed: Option<bool>,',
            );
        });

        it('boxes cross-category union fields (array/map/set.count -> Box<CountNode>)', () => {
            expect(getFromRenderMap(map, 'type_nodes/array_type_node.rs').content).toContain(
                'pub count: Box<CountNode>,',
            );
            expect(getFromRenderMap(map, 'type_nodes/map_type_node.rs').content).toContain(
                'pub count: Box<CountNode>,',
            );
            expect(getFromRenderMap(map, 'type_nodes/set_type_node.rs').content).toContain(
                'pub count: Box<CountNode>,',
            );
        });

        it('renders integer prefixes as a bare `IntegerTypeNode` and transforms as a bare `Vec<TransformNode>`', () => {
            const entry = getFromRenderMap(map, 'type_nodes/option_type_node.rs');
            expect(entry.content).toContain('pub prefix: IntegerTypeNode,');
            expect(entry.content).toContain('pub transforms: Vec<TransformNode>,');
        });

        it('escapes the `enum` field name as `r#enum`', () => {
            expect(getFromRenderMap(map, 'value_nodes/enum_value_node.rs').content).toContain(
                'pub r#enum: DefinedTypeLinkNode,',
            );
        });
    });
});
