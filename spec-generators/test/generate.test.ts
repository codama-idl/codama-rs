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
            'account_node.rs',
            'codama_version.rs',
            'constant_node.rs',
            'contextual_value_nodes/account_bump_value_node.rs',
            'contextual_value_nodes/account_field_value_node.rs',
            'contextual_value_nodes/account_value_node.rs',
            'contextual_value_nodes/argument_value_node.rs',
            'contextual_value_nodes/conditional_value_condition.rs',
            'contextual_value_nodes/conditional_value_node.rs',
            'contextual_value_nodes/contextual_value_node.rs',
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
            'contextual_value_nodes/resolver_dependency.rs',
            'contextual_value_nodes/resolver_value_node.rs',
            'count_nodes/count_node.rs',
            'count_nodes/fixed_count_node.rs',
            'count_nodes/mod.rs',
            'count_nodes/prefixed_count_node.rs',
            'count_nodes/remainder_count_node.rs',
            'defined_type_node.rs',
            'discriminator_nodes/constant_discriminator_node.rs',
            'discriminator_nodes/discriminator_node.rs',
            'discriminator_nodes/field_discriminator_node.rs',
            'discriminator_nodes/mod.rs',
            'discriminator_nodes/size_discriminator_node.rs',
            'display_nodes/amount_number_display_node.rs',
            'display_nodes/date_time_number_display_node.rs',
            'display_nodes/display_node.rs',
            'display_nodes/duration_number_display_node.rs',
            'display_nodes/enum_variant_display_node.rs',
            'display_nodes/instruction_account_display_node.rs',
            'display_nodes/instruction_display_node.rs',
            'display_nodes/mod.rs',
            'display_nodes/number_display_node.rs',
            'display_nodes/string_display_node.rs',
            'display_nodes/struct_field_display_node.rs',
            'error_node.rs',
            'event_node.rs',
            'instruction_account_node.rs',
            'instruction_argument_node.rs',
            'instruction_byte_delta_node.rs',
            'instruction_byte_delta_value.rs',
            'instruction_node.rs',
            'instruction_remaining_accounts_node.rs',
            'instruction_remaining_accounts_value.rs',
            'instruction_status_node.rs',
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
            'shared/bytes_encoding.rs',
            'shared/default_value_strategy.rs',
            'shared/display_skip.rs',
            'shared/endianness.rs',
            'shared/instruction_lifecycle.rs',
            'shared/is_signer.rs',
            'shared/mod.rs',
            'shared/number_format.rs',
            'shared/optional_account_strategy.rs',
            'shared/post_offset_strategy.rs',
            'shared/pre_offset_strategy.rs',
            'shared/program_origin.rs',
            'type_nodes/amount_type_node.rs',
            'type_nodes/array_type_node.rs',
            'type_nodes/boolean_type_node.rs',
            'type_nodes/bytes_type_node.rs',
            'type_nodes/date_time_type_node.rs',
            'type_nodes/enum_empty_variant_type_node.rs',
            'type_nodes/enum_struct_variant_type_node.rs',
            'type_nodes/enum_tuple_variant_type_node.rs',
            'type_nodes/enum_type_node.rs',
            'type_nodes/map_type_node.rs',
            'type_nodes/mod.rs',
            'type_nodes/number_type_node.rs',
            'type_nodes/option_type_node.rs',
            'type_nodes/public_key_type_node.rs',
            'type_nodes/remainder_option_type_node.rs',
            'type_nodes/set_type_node.rs',
            'type_nodes/sol_amount_type_node.rs',
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
            'value_nodes/enum_value_payload.rs',
            'value_nodes/injectable_number_value_node.rs',
            'value_nodes/injectable_string_value_node.rs',
            'value_nodes/injected_value_node.rs',
            'value_nodes/map_entry_value_node.rs',
            'value_nodes/map_value_node.rs',
            'value_nodes/mod.rs',
            'value_nodes/none_value_node.rs',
            'value_nodes/number_value_node.rs',
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
        for (const dir of [
            'contextual_value_nodes',
            'count_nodes',
            'discriminator_nodes',
            'display_nodes',
            'link_nodes',
            'pda_seed_nodes',
            'shared',
            'type_nodes',
            'value_nodes',
        ]) {
            expect(entry.content).toContain(`mod ${dir};`);
            expect(entry.content).toContain(`pub use ${dir}::*;`);
        }
    });

    describe('count category', () => {
        it('routes FixedCountNode through Node::Count and emits the u64 field plus #[derive(Copy, Default)]', () => {
            // `value: u64` is scalar (so Copy) and unconditionally
            // Default-able (so Default).
            const entry = getFromRenderMap(map, 'count_nodes/fixed_count_node.rs');
            expect(entry.content).toContain('#[node]');
            expect(entry.content).toContain('#[derive(Copy, Default)]');
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
            expect(size.content).toContain('#[derive(Copy, Default)]');
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

        it('derives Copy for scalar-only display nodes', () => {
            const entry = getFromRenderMap(map, 'display_nodes/string_display_node.rs');
            expect(entry.content).toContain('#[derive(Copy, Default)]');
        });

        it('boxes the optional injectable-union fields of AmountNumberDisplayNode', () => {
            const entry = getFromRenderMap(map, 'display_nodes/amount_number_display_node.rs');
            expect(entry.content).toContain('pub decimals: Box<Option<InjectableNumberValueNode>>,');
            expect(entry.content).toContain('pub unit: Box<Option<InjectableStringValueNode>>,');
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
            expect(entry.content).toContain('Number(NumberValueNode),');
            expect(entry.content).toContain('String(StringValueNode),');
            expect(entry.content).toContain('Boolean(BooleanValueNode),');
            // No HasName: variants like NumberValueNode have no name attribute.
            expect(entry.content).not.toContain('impl HasName for ConstantPdaSeedValue');
        });

        it('routes VariablePdaSeedNode through Node::PdaSeed with HasName + a bare Docs field', () => {
            const entry = getFromRenderMap(map, 'pda_seed_nodes/variable_pda_seed_node.rs');
            expect(entry.content).toContain('pub struct VariablePdaSeedNode {');
            expect(entry.content).toContain('pub name: CamelCaseString,');
            expect(entry.content).toContain('pub docs: Docs,');
            // Box rule: direct union fields are boxed.
            expect(entry.content).toContain('pub r#type: Box<TypeNode>,');
            expect(entry.content).toContain('impl HasName for VariablePdaSeedNode {');
        });

        it('emits the PdaSeedNode category union with Constant/Variable variants and no HasName', () => {
            const entry = getFromRenderMap(map, 'pda_seed_nodes/pda_seed_node.rs');
            expect(entry.content).toContain('pub enum PdaSeedNode {');
            expect(entry.content).toContain('Constant(ConstantPdaSeedNode),');
            expect(entry.content).toContain('Variable(VariablePdaSeedNode),');
            // ConstantPdaSeedNode has no name -> no union HasName impl.
            expect(entry.content).not.toContain('impl HasName for PdaSeedNode');
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
            expect(entry.content).toContain('pub value: Box<Option<EnumValuePayload>>,');
        });

        it('does NOT box Vec<union> fields (arrayValueNode / setValueNode / tupleValueNode)', () => {
            for (const file of ['array_value_node.rs', 'set_value_node.rs', 'tuple_value_node.rs']) {
                const entry = getFromRenderMap(map, `value_nodes/${file}`);
                expect(entry.content).toContain('pub items: Vec<ValueNode>,');
            }
        });

        it('renders numberValueNode.number as `Number` (float → Number) + #[derive(Copy)]', () => {
            const entry = getFromRenderMap(map, 'value_nodes/number_value_node.rs');
            expect(entry.content).toContain('pub struct NumberValueNode {');
            expect(entry.content).toContain('pub number: Number,');
            // `Number` is `Copy` but NOT `Default`, so the heuristic
            // derives `Copy` only — `numberValueNode` has a hand-written
            // `impl Default`.
            expect(entry.content).toContain('#[derive(Copy)]');
        });

        it('emits Copy/Default for noneValueNode (empty struct heuristic)', () => {
            const entry = getFromRenderMap(map, 'value_nodes/none_value_node.rs');
            expect(entry.content).toContain('#[derive(Copy, Default)]');
            expect(entry.content).toContain('pub struct NoneValueNode {}');
        });

        it('emits the EnumValuePayload inline-union enum with Struct/Tuple variants', () => {
            const entry = getFromRenderMap(map, 'value_nodes/enum_value_payload.rs');
            expect(entry.content).toContain('pub enum EnumValuePayload {');
            expect(entry.content).toContain('Struct(StructValueNode),');
            expect(entry.content).toContain('Tuple(TupleValueNode),');
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
            expect(entry.content).toContain('Resolver(ResolverValueNode),');
            // `#[registered]`-only: `pdaSeedValueNode`.
            expect(entry.content).toContain('#[registered]\nPdaSeed(PdaSeedValueNode),');
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
            // 23 ValueNode members keep their `Value` prefix when only
            // `Node` is stripped; `ProgramLink` is the lone non-value
            // member that drops to just `ProgramLink`.
            const entry = getFromRenderMap(map, 'contextual_value_nodes/instruction_input_value_node.rs');
            expect(entry.content).toContain('pub enum InstructionInputValueNode {');
            expect(entry.content).toContain('AccountValue(AccountValueNode),');
            expect(entry.content).toContain('ArgumentValue(ArgumentValueNode),');
            expect(entry.content).toContain('NumberValue(NumberValueNode),');
            expect(entry.content).toContain('ProgramLink(ProgramLinkNode),');
        });

        it('applies the box-all-union rule + Box<Option<T>> shapes on ConditionalValueNode', () => {
            const entry = getFromRenderMap(map, 'contextual_value_nodes/conditional_value_node.rs');
            expect(entry.content).toContain('pub condition: Box<ConditionalValueCondition>,');
            expect(entry.content).toContain('pub value: Box<Option<ValueNode>>,');
            expect(entry.content).toContain('pub if_true: Box<Option<InstructionInputValueNode>>,');
            expect(entry.content).toContain('pub if_false: Box<Option<InstructionInputValueNode>>,');
        });

        it('emits resolverValueNode.dependsOn as a bare `Vec` (matches the optional-array convention)', () => {
            const entry = getFromRenderMap(map, 'contextual_value_nodes/resolver_value_node.rs');
            expect(entry.content).toContain('pub depends_on: Vec<ResolverDependency>,');
            expect(entry.content).not.toContain('Option<Vec<ResolverDependency>>');
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
                'instruction_argument_node.rs',
                'instruction_byte_delta_node.rs',
                'instruction_node.rs',
                'instruction_remaining_accounts_node.rs',
                'instruction_status_node.rs',
                'pda_node.rs',
                'program_node.rs',
                'provided_node.rs',
                'root_node.rs',
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
            // `name` is a stringIdentifier, so the node still gets a HasName impl.
            expect(entry.content).toContain('impl HasName for ProvidedNode {');
        });

        it('renders instructionNode.provides as a bare `Vec<ProvidedNode>` and its optional display child', () => {
            const entry = getFromRenderMap(map, 'instruction_node.rs');
            expect(entry.content).toContain('pub provides: Vec<ProvidedNode>,');
            expect(entry.content).toContain('pub display: Option<InstructionDisplayNode>,');
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

        it('renders programNode.origin as `Option<ProgramOrigin>`', () => {
            expect(getFromRenderMap(map, 'program_node.rs').content).toContain('pub origin: Option<ProgramOrigin>,');
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
            expect(getFromRenderMap(map, 'instruction_status_node.rs').content).toContain(
                'pub message: Option<String>,',
            );
            expect(getFromRenderMap(map, 'instruction_node.rs').content).toContain(
                'pub optional_account_strategy: Option<OptionalAccountStrategy>,',
            );
        });

        it('boxes every direct (non-Vec) union field; leaves nestedUnion unboxed', () => {
            expect(getFromRenderMap(map, 'constant_node.rs').content).toContain('pub r#type: Box<TypeNode>,');
            expect(getFromRenderMap(map, 'constant_node.rs').content).toContain('pub value: Box<ValueNode>,');
            expect(getFromRenderMap(map, 'event_node.rs').content).toContain('pub data: Box<TypeNode>,');
            expect(getFromRenderMap(map, 'instruction_byte_delta_node.rs').content).toContain(
                'pub value: Box<InstructionByteDeltaValue>,',
            );
            expect(getFromRenderMap(map, 'instruction_account_node.rs').content).toContain(
                'pub default_value: Box<Option<InstructionInputValueNode>>,',
            );
            expect(getFromRenderMap(map, 'account_node.rs').content).toContain(
                'pub data: NestedTypeNode<StructTypeNode>,',
            );
        });

        it('derives InstructionByteDeltaValue variant names from the longest common suffix (`Node`)', () => {
            const entry = getFromRenderMap(map, 'instruction_byte_delta_value.rs');
            expect(entry.content).toContain('pub enum InstructionByteDeltaValue {');
            expect(entry.content).toContain('AccountLink(AccountLinkNode),');
            expect(entry.content).toContain('ArgumentValue(ArgumentValueNode),');
            expect(entry.content).toContain('NumberValue(NumberValueNode),');
            expect(entry.content).toContain('ResolverValue(ResolverValueNode),');
        });

        it('derives Default for all-Default-able nodes; suppresses it for nodes with opaque required fields', () => {
            expect(getFromRenderMap(map, 'program_node.rs').content).toContain('#[derive(Default)]');
            expect(getFromRenderMap(map, 'instruction_node.rs').content).toContain('#[derive(Default)]');
            expect(getFromRenderMap(map, 'error_node.rs').content).toContain('#[derive(Default)]');
            expect(getFromRenderMap(map, 'pda_node.rs').content).toContain('#[derive(Default)]');
            expect(getFromRenderMap(map, 'instruction_status_node.rs').content).not.toContain('Default');
            expect(getFromRenderMap(map, 'root_node.rs').content).not.toContain('Default');
            expect(getFromRenderMap(map, 'account_node.rs').content).not.toContain('Default');
        });

        it('emits a `CODAMA_VERSION` constant pinned to the spec version', () => {
            const entry = getFromRenderMap(map, 'codama_version.rs');
            expect(entry.content).toContain('pub const CODAMA_VERSION: &str = "1.8.0";');
        });
    });

    describe('shared/is_signer.rs (the sole v1 literalUnion shell)', () => {
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
    });

    describe('type category', () => {
        it('emits the 17 plain type nodes with `#[type_node]` and routes through `Node::Type`', () => {
            const entry = getFromRenderMap(map, 'type_nodes/amount_type_node.rs');
            expect(entry.content).toContain('#[type_node]');
            expect(entry.content).toContain('pub struct AmountTypeNode {');
            expect(entry.content).toContain('crate::Node::Type(val.into())');
        });

        it('emits the 3 enum-variant nodes and `structFieldTypeNode` with `#[node]` (not `#[type_node]`)', () => {
            const variant = getFromRenderMap(map, 'type_nodes/enum_empty_variant_type_node.rs');
            expect(variant.content).toContain('#[node]');
            expect(variant.content).not.toContain('#[type_node]');
            const structField = getFromRenderMap(map, 'type_nodes/struct_field_type_node.rs');
            expect(structField.content).toContain('#[node]');
            expect(structField.content).not.toContain('#[type_node]');
        });

        it('skips the 7 nestable wrapper nodes (fixedSize, sizePrefix, pre/postOffset, sentinel, hiddenPrefix/Suffix)', () => {
            for (const file of [
                'type_nodes/fixed_size_type_node.rs',
                'type_nodes/size_prefix_type_node.rs',
                'type_nodes/pre_offset_type_node.rs',
                'type_nodes/post_offset_type_node.rs',
                'type_nodes/sentinel_type_node.rs',
                'type_nodes/hidden_prefix_type_node.rs',
                'type_nodes/hidden_suffix_type_node.rs',
            ]) {
                expect(map.get(file)).toBeUndefined();
            }
        });

        it('skips ALL type-category unions (TypeNode, RegisteredTypeNode, EnumVariantTypeNode, StandaloneTypeNode)', () => {
            for (const file of [
                'type_nodes/type_node.rs',
                'type_nodes/registered_type_node.rs',
                'type_nodes/enum_variant_type_node.rs',
                'type_nodes/standalone_type_node.rs',
            ]) {
                expect(map.get(file)).toBeUndefined();
            }
        });

        it('aligns generated widths with the spec (decimals -> u32; enum-variant discriminator -> u32; option.fixed -> Option<bool>)', () => {
            expect(getFromRenderMap(map, 'type_nodes/amount_type_node.rs').content).toContain('pub decimals: u32,');
            expect(getFromRenderMap(map, 'type_nodes/enum_empty_variant_type_node.rs').content).toContain(
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

        it('renders nestedUnion(nestedTypeNode, X) fields as bare `NestedTypeNode<X>` (no box)', () => {
            expect(getFromRenderMap(map, 'type_nodes/amount_type_node.rs').content).toContain(
                'pub number: NestedTypeNode<NumberTypeNode>,',
            );
            expect(getFromRenderMap(map, 'type_nodes/enum_struct_variant_type_node.rs').content).toContain(
                'pub r#struct: NestedTypeNode<StructTypeNode>,',
            );
        });

        it('escapes the `struct` field name as `r#struct`', () => {
            expect(getFromRenderMap(map, 'type_nodes/enum_struct_variant_type_node.rs').content).toContain(
                'pub r#struct:',
            );
        });
    });
});
