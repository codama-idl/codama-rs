use crate::PathSegment::{Attribute, Item};
use crate::{
    walk_node, walk_registered_type, walk_type, AccountBumpValueNode, AccountDataValueNode,
    AccountLinkNode, AccountNode, AccountValueNode, AmountNumberDisplayNode, ArrayTypeNode,
    ArrayValueNode, BooleanTypeNode, BooleanValueNode, BytesTypeNode, BytesValueNode,
    ConditionalValueCondition, ConditionalValueNode, ConstantDiscriminatorNode, ConstantNode,
    ConstantPdaSeedNode, ConstantPdaSeedValue, ConstantValueNode, ContextualValueNode, CountNode,
    DataValueNode, DateTimeTypeNode, DefinedTypeLinkNode, DefinedTypeNode, DiscriminatorNode,
    DisplayNode, DurationTypeNode, EnumTypeNode, EnumValueNode, EnumVariantDisplayNode,
    EnumVariantTypeNode, ErrorNode, EventNode, FieldDiscriminatorNode, FixedCountNode,
    FixedPointTypeNode, FixedSizeTransformNode, FloatTypeNode, FloatValueNode,
    HiddenPrefixTransformNode, HiddenSuffixTransformNode, IdentityValueNode,
    InjectableIntegerValueNode, InjectableUnitValueNode, InjectedValueNode,
    InstructionAccountDisplayNode, InstructionAccountLinkNode, InstructionAccountNode,
    InstructionByteDeltaNode, InstructionByteDeltaValue, InstructionDisplayNode,
    InstructionInputValueNode, InstructionLinkNode, InstructionNode,
    InstructionRemainingAccountsNode, InstructionStatusNode, IntegerTypeNode, IntegerValueNode,
    LinkNode, MapEntryValueNode, MapTypeNode, MapValueNode, Node, NodePath, NoneValueNode,
    NumberDisplayNode, OptionTypeNode, PathSegment, PayerValueNode, PdaLinkNode, PdaNode,
    PdaSeedNode, PdaSeedValueNode, PdaSeedValueValue, PdaValueNode, PdaValuePda, PdaValueProgramId,
    PluginNode, PostOffsetTransformNode, PreOffsetTransformNode, PrefixedCountNode,
    ProgramIdValueNode, ProgramLinkNode, ProgramNode, ProvidedNode, PublicKeyTypeNode,
    PublicKeyValueNode, RegisteredContextualValueNode, RegisteredTypeNode, RegisteredValueNode,
    RemainderCountNode, RemainderOptionTypeNode, RootNode, SentinelCountNode,
    SentinelTransformNode, SetTypeNode, SetValueNode, SizeDiscriminatorNode,
    SizePrefixTransformNode, SomeValueNode, StringDisplayNode, StringTypeNode, StringValueNode,
    StructFieldDisplayNode, StructFieldTypeNode, StructFieldValueNode, StructTypeNode,
    StructValueNode, Text, TextNode, TransformNode, TupleTypeNode, TupleValueNode, TypeNode,
    UnitNumberDisplayNode, ValueLabelDisplayNode, ValueNode, VariablePdaSeedNode,
    ZeroableOptionTypeNode,
};

/// A read-only walk over a node tree, with one `visit_*` method per node kind and per union enum.
/// Each default `visit_*` method calls the `walk_*` function of the same name, which visits the children.
/// An override that does not call its `walk_*` function skips the children of its node.
/// A union method gets the path of the parent and the segment, as only the value knows the node to join.
/// Start a walk with `visitor.visit_root(&root, &NodePath::default())`.
pub trait NodeVisitor {
    /// Called by the `walk_*` function of each node kind, before it visits the children.
    fn enter_node<'a>(&mut self, _kind: &'static str, _path: &NodePath<'a>) {}

    fn visit_account<'a>(&mut self, node: &'a AccountNode, path: &NodePath<'a>) {
        walk_account(self, node, path);
    }

    fn visit_account_bump_value<'a>(
        &mut self,
        node: &'a AccountBumpValueNode,
        path: &NodePath<'a>,
    ) {
        walk_account_bump_value(self, node, path);
    }

    fn visit_account_data_value<'a>(
        &mut self,
        node: &'a AccountDataValueNode,
        path: &NodePath<'a>,
    ) {
        walk_account_data_value(self, node, path);
    }

    fn visit_account_link<'a>(&mut self, node: &'a AccountLinkNode, path: &NodePath<'a>) {
        walk_account_link(self, node, path);
    }

    fn visit_account_value<'a>(&mut self, node: &'a AccountValueNode, path: &NodePath<'a>) {
        walk_account_value(self, node, path);
    }

    fn visit_amount_number_display<'a>(
        &mut self,
        node: &'a AmountNumberDisplayNode,
        path: &NodePath<'a>,
    ) {
        walk_amount_number_display(self, node, path);
    }

    fn visit_array_type<'a>(&mut self, node: &'a ArrayTypeNode, path: &NodePath<'a>) {
        walk_array_type(self, node, path);
    }

    fn visit_array_value<'a>(&mut self, node: &'a ArrayValueNode, path: &NodePath<'a>) {
        walk_array_value(self, node, path);
    }

    fn visit_boolean_type<'a>(&mut self, node: &'a BooleanTypeNode, path: &NodePath<'a>) {
        walk_boolean_type(self, node, path);
    }

    fn visit_boolean_value<'a>(&mut self, node: &'a BooleanValueNode, path: &NodePath<'a>) {
        walk_boolean_value(self, node, path);
    }

    fn visit_bytes_type<'a>(&mut self, node: &'a BytesTypeNode, path: &NodePath<'a>) {
        walk_bytes_type(self, node, path);
    }

    fn visit_bytes_value<'a>(&mut self, node: &'a BytesValueNode, path: &NodePath<'a>) {
        walk_bytes_value(self, node, path);
    }

    fn visit_conditional_value<'a>(&mut self, node: &'a ConditionalValueNode, path: &NodePath<'a>) {
        walk_conditional_value(self, node, path);
    }

    fn visit_constant<'a>(&mut self, node: &'a ConstantNode, path: &NodePath<'a>) {
        walk_constant(self, node, path);
    }

    fn visit_constant_discriminator<'a>(
        &mut self,
        node: &'a ConstantDiscriminatorNode,
        path: &NodePath<'a>,
    ) {
        walk_constant_discriminator(self, node, path);
    }

    fn visit_constant_pda_seed<'a>(&mut self, node: &'a ConstantPdaSeedNode, path: &NodePath<'a>) {
        walk_constant_pda_seed(self, node, path);
    }

    fn visit_constant_value<'a>(&mut self, node: &'a ConstantValueNode, path: &NodePath<'a>) {
        walk_constant_value(self, node, path);
    }

    fn visit_data_value<'a>(&mut self, node: &'a DataValueNode, path: &NodePath<'a>) {
        walk_data_value(self, node, path);
    }

    fn visit_date_time_type<'a>(&mut self, node: &'a DateTimeTypeNode, path: &NodePath<'a>) {
        walk_date_time_type(self, node, path);
    }

    fn visit_defined_type<'a>(&mut self, node: &'a DefinedTypeNode, path: &NodePath<'a>) {
        walk_defined_type(self, node, path);
    }

    fn visit_defined_type_link<'a>(&mut self, node: &'a DefinedTypeLinkNode, path: &NodePath<'a>) {
        walk_defined_type_link(self, node, path);
    }

    fn visit_duration_type<'a>(&mut self, node: &'a DurationTypeNode, path: &NodePath<'a>) {
        walk_duration_type(self, node, path);
    }

    fn visit_enum_type<'a>(&mut self, node: &'a EnumTypeNode, path: &NodePath<'a>) {
        walk_enum_type(self, node, path);
    }

    fn visit_enum_value<'a>(&mut self, node: &'a EnumValueNode, path: &NodePath<'a>) {
        walk_enum_value(self, node, path);
    }

    fn visit_enum_variant_display<'a>(
        &mut self,
        node: &'a EnumVariantDisplayNode,
        path: &NodePath<'a>,
    ) {
        walk_enum_variant_display(self, node, path);
    }

    fn visit_enum_variant_type<'a>(&mut self, node: &'a EnumVariantTypeNode, path: &NodePath<'a>) {
        walk_enum_variant_type(self, node, path);
    }

    fn visit_error<'a>(&mut self, node: &'a ErrorNode, path: &NodePath<'a>) {
        walk_error(self, node, path);
    }

    fn visit_event<'a>(&mut self, node: &'a EventNode, path: &NodePath<'a>) {
        walk_event(self, node, path);
    }

    fn visit_field_discriminator<'a>(
        &mut self,
        node: &'a FieldDiscriminatorNode,
        path: &NodePath<'a>,
    ) {
        walk_field_discriminator(self, node, path);
    }

    fn visit_fixed_count<'a>(&mut self, node: &'a FixedCountNode, path: &NodePath<'a>) {
        walk_fixed_count(self, node, path);
    }

    fn visit_fixed_point_type<'a>(&mut self, node: &'a FixedPointTypeNode, path: &NodePath<'a>) {
        walk_fixed_point_type(self, node, path);
    }

    fn visit_fixed_size_transform<'a>(
        &mut self,
        node: &'a FixedSizeTransformNode,
        path: &NodePath<'a>,
    ) {
        walk_fixed_size_transform(self, node, path);
    }

    fn visit_float_type<'a>(&mut self, node: &'a FloatTypeNode, path: &NodePath<'a>) {
        walk_float_type(self, node, path);
    }

    fn visit_float_value<'a>(&mut self, node: &'a FloatValueNode, path: &NodePath<'a>) {
        walk_float_value(self, node, path);
    }

    fn visit_hidden_prefix_transform<'a>(
        &mut self,
        node: &'a HiddenPrefixTransformNode,
        path: &NodePath<'a>,
    ) {
        walk_hidden_prefix_transform(self, node, path);
    }

    fn visit_hidden_suffix_transform<'a>(
        &mut self,
        node: &'a HiddenSuffixTransformNode,
        path: &NodePath<'a>,
    ) {
        walk_hidden_suffix_transform(self, node, path);
    }

    fn visit_identity_value<'a>(&mut self, node: &'a IdentityValueNode, path: &NodePath<'a>) {
        walk_identity_value(self, node, path);
    }

    fn visit_injected_value<'a>(&mut self, node: &'a InjectedValueNode, path: &NodePath<'a>) {
        walk_injected_value(self, node, path);
    }

    fn visit_instruction<'a>(&mut self, node: &'a InstructionNode, path: &NodePath<'a>) {
        walk_instruction(self, node, path);
    }

    fn visit_instruction_account<'a>(
        &mut self,
        node: &'a InstructionAccountNode,
        path: &NodePath<'a>,
    ) {
        walk_instruction_account(self, node, path);
    }

    fn visit_instruction_account_display<'a>(
        &mut self,
        node: &'a InstructionAccountDisplayNode,
        path: &NodePath<'a>,
    ) {
        walk_instruction_account_display(self, node, path);
    }

    fn visit_instruction_account_link<'a>(
        &mut self,
        node: &'a InstructionAccountLinkNode,
        path: &NodePath<'a>,
    ) {
        walk_instruction_account_link(self, node, path);
    }

    fn visit_instruction_byte_delta<'a>(
        &mut self,
        node: &'a InstructionByteDeltaNode,
        path: &NodePath<'a>,
    ) {
        walk_instruction_byte_delta(self, node, path);
    }

    fn visit_instruction_display<'a>(
        &mut self,
        node: &'a InstructionDisplayNode,
        path: &NodePath<'a>,
    ) {
        walk_instruction_display(self, node, path);
    }

    fn visit_instruction_link<'a>(&mut self, node: &'a InstructionLinkNode, path: &NodePath<'a>) {
        walk_instruction_link(self, node, path);
    }

    fn visit_instruction_remaining_accounts<'a>(
        &mut self,
        node: &'a InstructionRemainingAccountsNode,
        path: &NodePath<'a>,
    ) {
        walk_instruction_remaining_accounts(self, node, path);
    }

    fn visit_instruction_status<'a>(
        &mut self,
        node: &'a InstructionStatusNode,
        path: &NodePath<'a>,
    ) {
        walk_instruction_status(self, node, path);
    }

    fn visit_integer_type<'a>(&mut self, node: &'a IntegerTypeNode, path: &NodePath<'a>) {
        walk_integer_type(self, node, path);
    }

    fn visit_integer_value<'a>(&mut self, node: &'a IntegerValueNode, path: &NodePath<'a>) {
        walk_integer_value(self, node, path);
    }

    fn visit_map_entry_value<'a>(&mut self, node: &'a MapEntryValueNode, path: &NodePath<'a>) {
        walk_map_entry_value(self, node, path);
    }

    fn visit_map_type<'a>(&mut self, node: &'a MapTypeNode, path: &NodePath<'a>) {
        walk_map_type(self, node, path);
    }

    fn visit_map_value<'a>(&mut self, node: &'a MapValueNode, path: &NodePath<'a>) {
        walk_map_value(self, node, path);
    }

    fn visit_none_value<'a>(&mut self, node: &'a NoneValueNode, path: &NodePath<'a>) {
        walk_none_value(self, node, path);
    }

    fn visit_option_type<'a>(&mut self, node: &'a OptionTypeNode, path: &NodePath<'a>) {
        walk_option_type(self, node, path);
    }

    fn visit_payer_value<'a>(&mut self, node: &'a PayerValueNode, path: &NodePath<'a>) {
        walk_payer_value(self, node, path);
    }

    fn visit_pda<'a>(&mut self, node: &'a PdaNode, path: &NodePath<'a>) {
        walk_pda(self, node, path);
    }

    fn visit_pda_link<'a>(&mut self, node: &'a PdaLinkNode, path: &NodePath<'a>) {
        walk_pda_link(self, node, path);
    }

    fn visit_pda_seed_value<'a>(&mut self, node: &'a PdaSeedValueNode, path: &NodePath<'a>) {
        walk_pda_seed_value(self, node, path);
    }

    fn visit_pda_value<'a>(&mut self, node: &'a PdaValueNode, path: &NodePath<'a>) {
        walk_pda_value(self, node, path);
    }

    fn visit_plugin<'a>(&mut self, node: &'a PluginNode, path: &NodePath<'a>) {
        walk_plugin(self, node, path);
    }

    fn visit_post_offset_transform<'a>(
        &mut self,
        node: &'a PostOffsetTransformNode,
        path: &NodePath<'a>,
    ) {
        walk_post_offset_transform(self, node, path);
    }

    fn visit_pre_offset_transform<'a>(
        &mut self,
        node: &'a PreOffsetTransformNode,
        path: &NodePath<'a>,
    ) {
        walk_pre_offset_transform(self, node, path);
    }

    fn visit_prefixed_count<'a>(&mut self, node: &'a PrefixedCountNode, path: &NodePath<'a>) {
        walk_prefixed_count(self, node, path);
    }

    fn visit_program<'a>(&mut self, node: &'a ProgramNode, path: &NodePath<'a>) {
        walk_program(self, node, path);
    }

    fn visit_program_id_value<'a>(&mut self, node: &'a ProgramIdValueNode, path: &NodePath<'a>) {
        walk_program_id_value(self, node, path);
    }

    fn visit_program_link<'a>(&mut self, node: &'a ProgramLinkNode, path: &NodePath<'a>) {
        walk_program_link(self, node, path);
    }

    fn visit_provided<'a>(&mut self, node: &'a ProvidedNode, path: &NodePath<'a>) {
        walk_provided(self, node, path);
    }

    fn visit_public_key_type<'a>(&mut self, node: &'a PublicKeyTypeNode, path: &NodePath<'a>) {
        walk_public_key_type(self, node, path);
    }

    fn visit_public_key_value<'a>(&mut self, node: &'a PublicKeyValueNode, path: &NodePath<'a>) {
        walk_public_key_value(self, node, path);
    }

    fn visit_remainder_count<'a>(&mut self, node: &'a RemainderCountNode, path: &NodePath<'a>) {
        walk_remainder_count(self, node, path);
    }

    fn visit_remainder_option_type<'a>(
        &mut self,
        node: &'a RemainderOptionTypeNode,
        path: &NodePath<'a>,
    ) {
        walk_remainder_option_type(self, node, path);
    }

    fn visit_root<'a>(&mut self, node: &'a RootNode, path: &NodePath<'a>) {
        walk_root(self, node, path);
    }

    fn visit_sentinel_count<'a>(&mut self, node: &'a SentinelCountNode, path: &NodePath<'a>) {
        walk_sentinel_count(self, node, path);
    }

    fn visit_sentinel_transform<'a>(
        &mut self,
        node: &'a SentinelTransformNode,
        path: &NodePath<'a>,
    ) {
        walk_sentinel_transform(self, node, path);
    }

    fn visit_set_type<'a>(&mut self, node: &'a SetTypeNode, path: &NodePath<'a>) {
        walk_set_type(self, node, path);
    }

    fn visit_set_value<'a>(&mut self, node: &'a SetValueNode, path: &NodePath<'a>) {
        walk_set_value(self, node, path);
    }

    fn visit_size_discriminator<'a>(
        &mut self,
        node: &'a SizeDiscriminatorNode,
        path: &NodePath<'a>,
    ) {
        walk_size_discriminator(self, node, path);
    }

    fn visit_size_prefix_transform<'a>(
        &mut self,
        node: &'a SizePrefixTransformNode,
        path: &NodePath<'a>,
    ) {
        walk_size_prefix_transform(self, node, path);
    }

    fn visit_some_value<'a>(&mut self, node: &'a SomeValueNode, path: &NodePath<'a>) {
        walk_some_value(self, node, path);
    }

    fn visit_string_display<'a>(&mut self, node: &'a StringDisplayNode, path: &NodePath<'a>) {
        walk_string_display(self, node, path);
    }

    fn visit_string_type<'a>(&mut self, node: &'a StringTypeNode, path: &NodePath<'a>) {
        walk_string_type(self, node, path);
    }

    fn visit_string_value<'a>(&mut self, node: &'a StringValueNode, path: &NodePath<'a>) {
        walk_string_value(self, node, path);
    }

    fn visit_struct_field_display<'a>(
        &mut self,
        node: &'a StructFieldDisplayNode,
        path: &NodePath<'a>,
    ) {
        walk_struct_field_display(self, node, path);
    }

    fn visit_struct_field_type<'a>(&mut self, node: &'a StructFieldTypeNode, path: &NodePath<'a>) {
        walk_struct_field_type(self, node, path);
    }

    fn visit_struct_field_value<'a>(
        &mut self,
        node: &'a StructFieldValueNode,
        path: &NodePath<'a>,
    ) {
        walk_struct_field_value(self, node, path);
    }

    fn visit_struct_type<'a>(&mut self, node: &'a StructTypeNode, path: &NodePath<'a>) {
        walk_struct_type(self, node, path);
    }

    fn visit_struct_value<'a>(&mut self, node: &'a StructValueNode, path: &NodePath<'a>) {
        walk_struct_value(self, node, path);
    }

    fn visit_text<'a>(&mut self, node: &'a TextNode, path: &NodePath<'a>) {
        walk_text(self, node, path);
    }

    fn visit_tuple_type<'a>(&mut self, node: &'a TupleTypeNode, path: &NodePath<'a>) {
        walk_tuple_type(self, node, path);
    }

    fn visit_tuple_value<'a>(&mut self, node: &'a TupleValueNode, path: &NodePath<'a>) {
        walk_tuple_value(self, node, path);
    }

    fn visit_unit_number_display<'a>(
        &mut self,
        node: &'a UnitNumberDisplayNode,
        path: &NodePath<'a>,
    ) {
        walk_unit_number_display(self, node, path);
    }

    fn visit_value_label_display<'a>(
        &mut self,
        node: &'a ValueLabelDisplayNode,
        path: &NodePath<'a>,
    ) {
        walk_value_label_display(self, node, path);
    }

    fn visit_variable_pda_seed<'a>(&mut self, node: &'a VariablePdaSeedNode, path: &NodePath<'a>) {
        walk_variable_pda_seed(self, node, path);
    }

    fn visit_zeroable_option_type<'a>(
        &mut self,
        node: &'a ZeroableOptionTypeNode,
        path: &NodePath<'a>,
    ) {
        walk_zeroable_option_type(self, node, path);
    }

    fn visit_conditional_value_condition<'a>(
        &mut self,
        node: &'a ConditionalValueCondition,
        path: &NodePath<'a>,
        segment: PathSegment,
    ) {
        walk_conditional_value_condition(self, node, path, segment);
    }

    fn visit_constant_pda_seed_value<'a>(
        &mut self,
        node: &'a ConstantPdaSeedValue,
        path: &NodePath<'a>,
        segment: PathSegment,
    ) {
        walk_constant_pda_seed_value(self, node, path, segment);
    }

    fn visit_contextual_value<'a>(
        &mut self,
        node: &'a ContextualValueNode,
        path: &NodePath<'a>,
        segment: PathSegment,
    ) {
        walk_contextual_value(self, node, path, segment);
    }

    fn visit_count<'a>(&mut self, node: &'a CountNode, path: &NodePath<'a>, segment: PathSegment) {
        walk_count(self, node, path, segment);
    }

    fn visit_discriminator<'a>(
        &mut self,
        node: &'a DiscriminatorNode,
        path: &NodePath<'a>,
        segment: PathSegment,
    ) {
        walk_discriminator(self, node, path, segment);
    }

    fn visit_display<'a>(
        &mut self,
        node: &'a DisplayNode,
        path: &NodePath<'a>,
        segment: PathSegment,
    ) {
        walk_display(self, node, path, segment);
    }

    fn visit_injectable_integer_value<'a>(
        &mut self,
        node: &'a InjectableIntegerValueNode,
        path: &NodePath<'a>,
        segment: PathSegment,
    ) {
        walk_injectable_integer_value(self, node, path, segment);
    }

    fn visit_injectable_unit_value<'a>(
        &mut self,
        node: &'a InjectableUnitValueNode,
        path: &NodePath<'a>,
        segment: PathSegment,
    ) {
        walk_injectable_unit_value(self, node, path, segment);
    }

    fn visit_instruction_byte_delta_value<'a>(
        &mut self,
        node: &'a InstructionByteDeltaValue,
        path: &NodePath<'a>,
        segment: PathSegment,
    ) {
        walk_instruction_byte_delta_value(self, node, path, segment);
    }

    fn visit_instruction_input_value<'a>(
        &mut self,
        node: &'a InstructionInputValueNode,
        path: &NodePath<'a>,
        segment: PathSegment,
    ) {
        walk_instruction_input_value(self, node, path, segment);
    }

    fn visit_link<'a>(&mut self, node: &'a LinkNode, path: &NodePath<'a>, segment: PathSegment) {
        walk_link(self, node, path, segment);
    }

    fn visit_node<'a>(&mut self, node: &'a Node, path: &NodePath<'a>, segment: PathSegment) {
        walk_node(self, node, path, segment);
    }

    fn visit_number_display<'a>(
        &mut self,
        node: &'a NumberDisplayNode,
        path: &NodePath<'a>,
        segment: PathSegment,
    ) {
        walk_number_display(self, node, path, segment);
    }

    fn visit_pda_seed<'a>(
        &mut self,
        node: &'a PdaSeedNode,
        path: &NodePath<'a>,
        segment: PathSegment,
    ) {
        walk_pda_seed(self, node, path, segment);
    }

    fn visit_pda_seed_value_value<'a>(
        &mut self,
        node: &'a PdaSeedValueValue,
        path: &NodePath<'a>,
        segment: PathSegment,
    ) {
        walk_pda_seed_value_value(self, node, path, segment);
    }

    fn visit_pda_value_pda<'a>(
        &mut self,
        node: &'a PdaValuePda,
        path: &NodePath<'a>,
        segment: PathSegment,
    ) {
        walk_pda_value_pda(self, node, path, segment);
    }

    fn visit_pda_value_program_id<'a>(
        &mut self,
        node: &'a PdaValueProgramId,
        path: &NodePath<'a>,
        segment: PathSegment,
    ) {
        walk_pda_value_program_id(self, node, path, segment);
    }

    fn visit_registered_contextual_value<'a>(
        &mut self,
        node: &'a RegisteredContextualValueNode,
        path: &NodePath<'a>,
        segment: PathSegment,
    ) {
        walk_registered_contextual_value(self, node, path, segment);
    }

    fn visit_registered_type<'a>(
        &mut self,
        node: &'a RegisteredTypeNode,
        path: &NodePath<'a>,
        segment: PathSegment,
    ) {
        walk_registered_type(self, node, path, segment);
    }

    fn visit_registered_value<'a>(
        &mut self,
        node: &'a RegisteredValueNode,
        path: &NodePath<'a>,
        segment: PathSegment,
    ) {
        walk_registered_value(self, node, path, segment);
    }

    fn visit_transform<'a>(
        &mut self,
        node: &'a TransformNode,
        path: &NodePath<'a>,
        segment: PathSegment,
    ) {
        walk_transform(self, node, path, segment);
    }

    fn visit_type<'a>(&mut self, node: &'a TypeNode, path: &NodePath<'a>, segment: PathSegment) {
        walk_type(self, node, path, segment);
    }

    fn visit_value<'a>(&mut self, node: &'a ValueNode, path: &NodePath<'a>, segment: PathSegment) {
        walk_value(self, node, path, segment);
    }
}

pub fn walk_account<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a AccountNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("accountNode", path);
    if let Some(Text::Node(child)) = &*node.docs {
        visitor.visit_text(child, &path.join(Attribute("docs"), child));
    }
    visitor.visit_type(&node.data, path, Attribute("data"));
    if let Some(child) = &node.pda {
        visitor.visit_pda_link(child, &path.join_named(Attribute("pda"), child));
    }
    for (index, child) in node.discriminators.iter().enumerate() {
        visitor.visit_discriminator(child, path, Item("discriminators", index));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_account_bump_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a AccountBumpValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("accountBumpValueNode", path);
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_account_data_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a AccountDataValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("accountDataValueNode", path);
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_account_link<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a AccountLinkNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("accountLinkNode", path);
    if let Some(child) = &node.program {
        visitor.visit_program_link(child, &path.join_named(Attribute("program"), child));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_account_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a AccountValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("accountValueNode", path);
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_amount_number_display<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a AmountNumberDisplayNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("amountNumberDisplayNode", path);
    visitor.visit_injectable_integer_value(&node.decimals, path, Attribute("decimals"));
    if let Some(child) = &*node.unit {
        visitor.visit_injectable_unit_value(child, path, Attribute("unit"));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_array_type<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a ArrayTypeNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("arrayTypeNode", path);
    visitor.visit_type(&node.item, path, Attribute("item"));
    visitor.visit_count(&node.count, path, Attribute("count"));
    for (index, child) in node.transforms.iter().enumerate() {
        visitor.visit_transform(child, path, Item("transforms", index));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_array_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a ArrayValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("arrayValueNode", path);
    for (index, child) in node.items.iter().enumerate() {
        visitor.visit_value(child, path, Item("items", index));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_boolean_type<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a BooleanTypeNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("booleanTypeNode", path);
    visitor.visit_integer_type(&node.size, &path.join(Attribute("size"), &node.size));
    for (index, child) in node.transforms.iter().enumerate() {
        visitor.visit_transform(child, path, Item("transforms", index));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_boolean_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a BooleanValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("booleanValueNode", path);
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_bytes_type<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a BytesTypeNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("bytesTypeNode", path);
    for (index, child) in node.transforms.iter().enumerate() {
        visitor.visit_transform(child, path, Item("transforms", index));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_bytes_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a BytesValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("bytesValueNode", path);
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_conditional_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a ConditionalValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("conditionalValueNode", path);
    visitor.visit_conditional_value_condition(&node.condition, path, Attribute("condition"));
    if let Some(child) = &*node.value {
        visitor.visit_value(child, path, Attribute("value"));
    }
    if let Some(child) = &*node.if_true {
        visitor.visit_instruction_input_value(child, path, Attribute("ifTrue"));
    }
    if let Some(child) = &*node.if_false {
        visitor.visit_instruction_input_value(child, path, Attribute("ifFalse"));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_constant<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a ConstantNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("constantNode", path);
    if let Some(Text::Node(child)) = &*node.docs {
        visitor.visit_text(child, &path.join(Attribute("docs"), child));
    }
    visitor.visit_type(&node.r#type, path, Attribute("type"));
    visitor.visit_value(&node.value, path, Attribute("value"));
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_constant_discriminator<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a ConstantDiscriminatorNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("constantDiscriminatorNode", path);
    visitor.visit_constant_value(
        &node.constant,
        &path.join(Attribute("constant"), &node.constant),
    );
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_constant_pda_seed<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a ConstantPdaSeedNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("constantPdaSeedNode", path);
    visitor.visit_type(&node.r#type, path, Attribute("type"));
    visitor.visit_constant_pda_seed_value(&node.value, path, Attribute("value"));
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_constant_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a ConstantValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("constantValueNode", path);
    visitor.visit_type(&node.r#type, path, Attribute("type"));
    visitor.visit_value(&node.value, path, Attribute("value"));
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_data_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a DataValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("dataValueNode", path);
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_date_time_type<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a DateTimeTypeNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("dateTimeTypeNode", path);
    visitor.visit_integer_type(&node.number, &path.join(Attribute("number"), &node.number));
    for (index, child) in node.transforms.iter().enumerate() {
        visitor.visit_transform(child, path, Item("transforms", index));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_defined_type<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a DefinedTypeNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("definedTypeNode", path);
    if let Some(Text::Node(child)) = &*node.docs {
        visitor.visit_text(child, &path.join(Attribute("docs"), child));
    }
    visitor.visit_type(&node.r#type, path, Attribute("type"));
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_defined_type_link<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a DefinedTypeLinkNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("definedTypeLinkNode", path);
    if let Some(child) = &node.program {
        visitor.visit_program_link(child, &path.join_named(Attribute("program"), child));
    }
    for (index, child) in node.transforms.iter().enumerate() {
        visitor.visit_transform(child, path, Item("transforms", index));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_duration_type<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a DurationTypeNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("durationTypeNode", path);
    visitor.visit_integer_type(&node.number, &path.join(Attribute("number"), &node.number));
    for (index, child) in node.transforms.iter().enumerate() {
        visitor.visit_transform(child, path, Item("transforms", index));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_enum_type<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a EnumTypeNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("enumTypeNode", path);
    for (index, child) in node.variants.iter().enumerate() {
        visitor.visit_enum_variant_type(child, &path.join_named(Item("variants", index), child));
    }
    visitor.visit_integer_type(&node.size, &path.join(Attribute("size"), &node.size));
    for (index, child) in node.transforms.iter().enumerate() {
        visitor.visit_transform(child, path, Item("transforms", index));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_enum_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a EnumValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("enumValueNode", path);
    visitor.visit_defined_type_link(
        &node.r#enum,
        &path.join_named(Attribute("enum"), &node.r#enum),
    );
    if let Some(child) = &*node.value {
        visitor.visit_value(child, path, Attribute("value"));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_enum_variant_display<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a EnumVariantDisplayNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("enumVariantDisplayNode", path);
    if let Some(Text::Node(child)) = &node.label {
        visitor.visit_text(child, &path.join(Attribute("label"), child));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_enum_variant_type<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a EnumVariantTypeNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("enumVariantTypeNode", path);
    if let Some(Text::Node(child)) = &*node.docs {
        visitor.visit_text(child, &path.join(Attribute("docs"), child));
    }
    if let Some(child) = &*node.data {
        visitor.visit_type(child, path, Attribute("data"));
    }
    if let Some(child) = &node.display {
        visitor.visit_enum_variant_display(child, &path.join(Attribute("display"), child));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_error<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a ErrorNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("errorNode", path);
    if let Text::Node(child) = &node.message {
        visitor.visit_text(child, &path.join(Attribute("message"), child));
    }
    if let Some(Text::Node(child)) = &*node.docs {
        visitor.visit_text(child, &path.join(Attribute("docs"), child));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_event<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a EventNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("eventNode", path);
    if let Some(Text::Node(child)) = &*node.docs {
        visitor.visit_text(child, &path.join(Attribute("docs"), child));
    }
    visitor.visit_type(&node.data, path, Attribute("data"));
    for (index, child) in node.discriminators.iter().enumerate() {
        visitor.visit_discriminator(child, path, Item("discriminators", index));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_field_discriminator<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a FieldDiscriminatorNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("fieldDiscriminatorNode", path);
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_fixed_count<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a FixedCountNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("fixedCountNode", path);
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_fixed_point_type<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a FixedPointTypeNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("fixedPointTypeNode", path);
    visitor.visit_integer_type(&node.number, &path.join(Attribute("number"), &node.number));
    if let Some(child) = &node.display {
        visitor.visit_unit_number_display(child, &path.join(Attribute("display"), child));
    }
    for (index, child) in node.transforms.iter().enumerate() {
        visitor.visit_transform(child, path, Item("transforms", index));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_fixed_size_transform<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a FixedSizeTransformNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("fixedSizeTransformNode", path);
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_float_type<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a FloatTypeNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("floatTypeNode", path);
    if let Some(child) = &node.display {
        visitor.visit_unit_number_display(child, &path.join(Attribute("display"), child));
    }
    for (index, child) in node.transforms.iter().enumerate() {
        visitor.visit_transform(child, path, Item("transforms", index));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_float_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a FloatValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("floatValueNode", path);
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_hidden_prefix_transform<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a HiddenPrefixTransformNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("hiddenPrefixTransformNode", path);
    for (index, child) in node.prefix.iter().enumerate() {
        visitor.visit_constant_value(child, &path.join(Item("prefix", index), child));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_hidden_suffix_transform<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a HiddenSuffixTransformNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("hiddenSuffixTransformNode", path);
    for (index, child) in node.suffix.iter().enumerate() {
        visitor.visit_constant_value(child, &path.join(Item("suffix", index), child));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_identity_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a IdentityValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("identityValueNode", path);
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_injected_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a InjectedValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("injectedValueNode", path);
    if let Some(child) = &*node.fallback {
        visitor.visit_value(child, path, Attribute("fallback"));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_instruction<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a InstructionNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("instructionNode", path);
    if let Some(Text::Node(child)) = &*node.docs {
        visitor.visit_text(child, &path.join(Attribute("docs"), child));
    }
    for (index, child) in node.accounts.iter().enumerate() {
        visitor.visit_instruction_account(child, &path.join_named(Item("accounts", index), child));
    }
    if let Some(child) = &*node.data {
        visitor.visit_type(child, path, Attribute("data"));
    }
    if let Some(child) = &*node.return_data {
        visitor.visit_type(child, path, Attribute("returnData"));
    }
    for (index, child) in node.remaining_accounts.iter().enumerate() {
        visitor.visit_instruction_remaining_accounts(
            child,
            &path.join_named(Item("remainingAccounts", index), child),
        );
    }
    for (index, child) in node.byte_deltas.iter().enumerate() {
        visitor.visit_instruction_byte_delta(child, &path.join(Item("byteDeltas", index), child));
    }
    for (index, child) in node.discriminators.iter().enumerate() {
        visitor.visit_discriminator(child, path, Item("discriminators", index));
    }
    if let Some(child) = &node.status {
        visitor.visit_instruction_status(child, &path.join(Attribute("status"), child));
    }
    for (index, child) in node.sub_instructions.iter().enumerate() {
        visitor.visit_instruction(
            child,
            &path.join_named(Item("subInstructions", index), child),
        );
    }
    for (index, child) in node.provides.iter().enumerate() {
        visitor.visit_provided(child, &path.join_named(Item("provides", index), child));
    }
    if let Some(child) = &node.display {
        visitor.visit_instruction_display(child, &path.join(Attribute("display"), child));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_instruction_account<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a InstructionAccountNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("instructionAccountNode", path);
    if let Some(Text::Node(child)) = &*node.docs {
        visitor.visit_text(child, &path.join(Attribute("docs"), child));
    }
    if let Some(child) = &*node.default_value {
        visitor.visit_instruction_input_value(child, path, Attribute("defaultValue"));
    }
    if let Some(child) = &node.account_link {
        visitor.visit_account_link(child, &path.join_named(Attribute("accountLink"), child));
    }
    if let Some(child) = &node.display {
        visitor.visit_instruction_account_display(child, &path.join(Attribute("display"), child));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_instruction_account_display<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a InstructionAccountDisplayNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("instructionAccountDisplayNode", path);
    if let Some(Text::Node(child)) = &node.label {
        visitor.visit_text(child, &path.join(Attribute("label"), child));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_instruction_account_link<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a InstructionAccountLinkNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("instructionAccountLinkNode", path);
    if let Some(child) = &node.instruction {
        visitor.visit_instruction_link(child, &path.join_named(Attribute("instruction"), child));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_instruction_byte_delta<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a InstructionByteDeltaNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("instructionByteDeltaNode", path);
    visitor.visit_instruction_byte_delta_value(&node.value, path, Attribute("value"));
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_instruction_display<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a InstructionDisplayNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("instructionDisplayNode", path);
    if let Some(Text::Node(child)) = &node.intent {
        visitor.visit_text(child, &path.join(Attribute("intent"), child));
    }
    if let Some(Text::Node(child)) = &node.interpolated_intent {
        visitor.visit_text(child, &path.join(Attribute("interpolatedIntent"), child));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_instruction_link<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a InstructionLinkNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("instructionLinkNode", path);
    if let Some(child) = &node.program {
        visitor.visit_program_link(child, &path.join_named(Attribute("program"), child));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_instruction_remaining_accounts<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a InstructionRemainingAccountsNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("instructionRemainingAccountsNode", path);
    if let Some(Text::Node(child)) = &*node.docs {
        visitor.visit_text(child, &path.join(Attribute("docs"), child));
    }
    if let Some(child) = &node.display {
        visitor.visit_instruction_account_display(child, &path.join(Attribute("display"), child));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_instruction_status<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a InstructionStatusNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("instructionStatusNode", path);
    if let Some(Text::Node(child)) = &node.message {
        visitor.visit_text(child, &path.join(Attribute("message"), child));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_integer_type<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a IntegerTypeNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("integerTypeNode", path);
    if let Some(child) = &*node.display {
        visitor.visit_number_display(child, path, Attribute("display"));
    }
    for (index, child) in node.transforms.iter().enumerate() {
        visitor.visit_transform(child, path, Item("transforms", index));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_integer_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a IntegerValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("integerValueNode", path);
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_map_entry_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a MapEntryValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("mapEntryValueNode", path);
    visitor.visit_value(&node.key, path, Attribute("key"));
    visitor.visit_value(&node.value, path, Attribute("value"));
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_map_type<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a MapTypeNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("mapTypeNode", path);
    visitor.visit_type(&node.key, path, Attribute("key"));
    visitor.visit_type(&node.value, path, Attribute("value"));
    visitor.visit_count(&node.count, path, Attribute("count"));
    for (index, child) in node.transforms.iter().enumerate() {
        visitor.visit_transform(child, path, Item("transforms", index));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_map_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a MapValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("mapValueNode", path);
    for (index, child) in node.entries.iter().enumerate() {
        visitor.visit_map_entry_value(child, &path.join(Item("entries", index), child));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_none_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a NoneValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("noneValueNode", path);
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_option_type<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a OptionTypeNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("optionTypeNode", path);
    visitor.visit_type(&node.item, path, Attribute("item"));
    visitor.visit_integer_type(&node.prefix, &path.join(Attribute("prefix"), &node.prefix));
    for (index, child) in node.transforms.iter().enumerate() {
        visitor.visit_transform(child, path, Item("transforms", index));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_payer_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a PayerValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("payerValueNode", path);
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_pda<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a PdaNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("pdaNode", path);
    if let Some(Text::Node(child)) = &*node.docs {
        visitor.visit_text(child, &path.join(Attribute("docs"), child));
    }
    for (index, child) in node.seeds.iter().enumerate() {
        visitor.visit_pda_seed(child, path, Item("seeds", index));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_pda_link<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a PdaLinkNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("pdaLinkNode", path);
    if let Some(child) = &node.program {
        visitor.visit_program_link(child, &path.join_named(Attribute("program"), child));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_pda_seed_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a PdaSeedValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("pdaSeedValueNode", path);
    visitor.visit_pda_seed_value_value(&node.value, path, Attribute("value"));
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_pda_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a PdaValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("pdaValueNode", path);
    visitor.visit_pda_value_pda(&node.pda, path, Attribute("pda"));
    for (index, child) in node.seeds.iter().enumerate() {
        visitor.visit_pda_seed_value(child, &path.join_named(Item("seeds", index), child));
    }
    if let Some(child) = &*node.program_id {
        visitor.visit_pda_value_program_id(child, path, Attribute("programId"));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_plugin<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a PluginNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("pluginNode", path);
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_post_offset_transform<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a PostOffsetTransformNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("postOffsetTransformNode", path);
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_pre_offset_transform<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a PreOffsetTransformNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("preOffsetTransformNode", path);
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_prefixed_count<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a PrefixedCountNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("prefixedCountNode", path);
    visitor.visit_integer_type(&node.prefix, &path.join(Attribute("prefix"), &node.prefix));
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_program<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a ProgramNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("programNode", path);
    if let Some(Text::Node(child)) = &*node.docs {
        visitor.visit_text(child, &path.join(Attribute("docs"), child));
    }
    for (index, child) in node.accounts.iter().enumerate() {
        visitor.visit_account(child, &path.join_named(Item("accounts", index), child));
    }
    for (index, child) in node.instructions.iter().enumerate() {
        visitor.visit_instruction(child, &path.join_named(Item("instructions", index), child));
    }
    for (index, child) in node.defined_types.iter().enumerate() {
        visitor.visit_defined_type(child, &path.join_named(Item("definedTypes", index), child));
    }
    for (index, child) in node.pdas.iter().enumerate() {
        visitor.visit_pda(child, &path.join_named(Item("pdas", index), child));
    }
    for (index, child) in node.events.iter().enumerate() {
        visitor.visit_event(child, &path.join_named(Item("events", index), child));
    }
    for (index, child) in node.errors.iter().enumerate() {
        visitor.visit_error(child, &path.join_named(Item("errors", index), child));
    }
    for (index, child) in node.constants.iter().enumerate() {
        visitor.visit_constant(child, &path.join_named(Item("constants", index), child));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_program_id_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a ProgramIdValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("programIdValueNode", path);
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_program_link<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a ProgramLinkNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("programLinkNode", path);
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_provided<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a ProvidedNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("providedNode", path);
    visitor.visit_node(&node.node, path, Attribute("node"));
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_public_key_type<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a PublicKeyTypeNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("publicKeyTypeNode", path);
    if let Some(child) = &node.account_link {
        visitor.visit_account_link(child, &path.join_named(Attribute("accountLink"), child));
    }
    for (index, child) in node.transforms.iter().enumerate() {
        visitor.visit_transform(child, path, Item("transforms", index));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_public_key_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a PublicKeyValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("publicKeyValueNode", path);
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_remainder_count<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a RemainderCountNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("remainderCountNode", path);
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_remainder_option_type<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a RemainderOptionTypeNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("remainderOptionTypeNode", path);
    visitor.visit_type(&node.item, path, Attribute("item"));
    for (index, child) in node.transforms.iter().enumerate() {
        visitor.visit_transform(child, path, Item("transforms", index));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_root<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a RootNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("rootNode", path);
    visitor.visit_program(
        &node.program,
        &path.join_named(Attribute("program"), &node.program),
    );
    for (index, child) in node.additional_programs.iter().enumerate() {
        visitor.visit_program(
            child,
            &path.join_named(Item("additionalPrograms", index), child),
        );
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_sentinel_count<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a SentinelCountNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("sentinelCountNode", path);
    visitor.visit_constant_value(
        &node.sentinel,
        &path.join(Attribute("sentinel"), &node.sentinel),
    );
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_sentinel_transform<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a SentinelTransformNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("sentinelTransformNode", path);
    visitor.visit_constant_value(
        &node.sentinel,
        &path.join(Attribute("sentinel"), &node.sentinel),
    );
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_set_type<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a SetTypeNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("setTypeNode", path);
    visitor.visit_type(&node.item, path, Attribute("item"));
    visitor.visit_count(&node.count, path, Attribute("count"));
    for (index, child) in node.transforms.iter().enumerate() {
        visitor.visit_transform(child, path, Item("transforms", index));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_set_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a SetValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("setValueNode", path);
    for (index, child) in node.items.iter().enumerate() {
        visitor.visit_value(child, path, Item("items", index));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_size_discriminator<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a SizeDiscriminatorNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("sizeDiscriminatorNode", path);
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_size_prefix_transform<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a SizePrefixTransformNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("sizePrefixTransformNode", path);
    visitor.visit_integer_type(&node.prefix, &path.join(Attribute("prefix"), &node.prefix));
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_some_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a SomeValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("someValueNode", path);
    visitor.visit_value(&node.value, path, Attribute("value"));
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_string_display<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a StringDisplayNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("stringDisplayNode", path);
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_string_type<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a StringTypeNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("stringTypeNode", path);
    if let Some(child) = &node.display {
        visitor.visit_string_display(child, &path.join(Attribute("display"), child));
    }
    for (index, child) in node.transforms.iter().enumerate() {
        visitor.visit_transform(child, path, Item("transforms", index));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_string_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a StringValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("stringValueNode", path);
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_struct_field_display<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a StructFieldDisplayNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("structFieldDisplayNode", path);
    if let Some(Text::Node(child)) = &node.label {
        visitor.visit_text(child, &path.join(Attribute("label"), child));
    }
    if let Some(Text::Node(child)) = &node.flatten_prefix {
        visitor.visit_text(child, &path.join(Attribute("flattenPrefix"), child));
    }
    for (index, child) in node.value_labels.iter().enumerate() {
        visitor.visit_value_label_display(child, &path.join(Item("valueLabels", index), child));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_struct_field_type<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a StructFieldTypeNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("structFieldTypeNode", path);
    if let Some(Text::Node(child)) = &*node.docs {
        visitor.visit_text(child, &path.join(Attribute("docs"), child));
    }
    visitor.visit_type(&node.r#type, path, Attribute("type"));
    if let Some(child) = &*node.default_value {
        visitor.visit_value(child, path, Attribute("defaultValue"));
    }
    if let Some(child) = &node.display {
        visitor.visit_struct_field_display(child, &path.join(Attribute("display"), child));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_struct_field_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a StructFieldValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("structFieldValueNode", path);
    visitor.visit_value(&node.value, path, Attribute("value"));
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_struct_type<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a StructTypeNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("structTypeNode", path);
    for (index, child) in node.fields.iter().enumerate() {
        visitor.visit_struct_field_type(child, &path.join_named(Item("fields", index), child));
    }
    for (index, child) in node.transforms.iter().enumerate() {
        visitor.visit_transform(child, path, Item("transforms", index));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_struct_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a StructValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("structValueNode", path);
    for (index, child) in node.fields.iter().enumerate() {
        visitor.visit_struct_field_value(child, &path.join_named(Item("fields", index), child));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_text<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a TextNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("textNode", path);
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_tuple_type<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a TupleTypeNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("tupleTypeNode", path);
    for (index, child) in node.items.iter().enumerate() {
        visitor.visit_type(child, path, Item("items", index));
    }
    for (index, child) in node.transforms.iter().enumerate() {
        visitor.visit_transform(child, path, Item("transforms", index));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_tuple_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a TupleValueNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("tupleValueNode", path);
    for (index, child) in node.items.iter().enumerate() {
        visitor.visit_value(child, path, Item("items", index));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_unit_number_display<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a UnitNumberDisplayNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("unitNumberDisplayNode", path);
    visitor.visit_injectable_unit_value(&node.unit, path, Attribute("unit"));
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_value_label_display<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a ValueLabelDisplayNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("valueLabelDisplayNode", path);
    if let Text::Node(child) = &node.label {
        visitor.visit_text(child, &path.join(Attribute("label"), child));
    }
    visitor.visit_value(&node.value, path, Attribute("value"));
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_variable_pda_seed<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a VariablePdaSeedNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("variablePdaSeedNode", path);
    if let Some(Text::Node(child)) = &*node.docs {
        visitor.visit_text(child, &path.join(Attribute("docs"), child));
    }
    visitor.visit_type(&node.r#type, path, Attribute("type"));
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_zeroable_option_type<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a ZeroableOptionTypeNode,
    path: &NodePath<'a>,
) {
    visitor.enter_node("zeroableOptionTypeNode", path);
    visitor.visit_type(&node.item, path, Attribute("item"));
    if let Some(child) = &node.zero_value {
        visitor.visit_constant_value(child, &path.join(Attribute("zeroValue"), child));
    }
    for (index, child) in node.transforms.iter().enumerate() {
        visitor.visit_transform(child, path, Item("transforms", index));
    }
    for (index, child) in node.plugins.iter().enumerate() {
        visitor.visit_plugin(child, &path.join(Item("plugins", index), child));
    }
}

pub fn walk_conditional_value_condition<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a ConditionalValueCondition,
    path: &NodePath<'a>,
    segment: PathSegment,
) {
    match node {
        ConditionalValueCondition::Account(node) => {
            visitor.visit_account_value(node, &path.join_named(segment, node))
        }
        ConditionalValueCondition::Data(node) => {
            visitor.visit_data_value(node, &path.join(segment, node))
        }
    }
}

pub fn walk_constant_pda_seed_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a ConstantPdaSeedValue,
    path: &NodePath<'a>,
    segment: PathSegment,
) {
    match node {
        ConstantPdaSeedValue::Array(node) => {
            visitor.visit_array_value(node, &path.join(segment, node))
        }
        ConstantPdaSeedValue::Boolean(node) => {
            visitor.visit_boolean_value(node, &path.join(segment, node))
        }
        ConstantPdaSeedValue::Bytes(node) => {
            visitor.visit_bytes_value(node, &path.join(segment, node))
        }
        ConstantPdaSeedValue::Constant(node) => {
            visitor.visit_constant_value(node, &path.join(segment, node))
        }
        ConstantPdaSeedValue::Enum(node) => {
            visitor.visit_enum_value(node, &path.join(segment, node))
        }
        ConstantPdaSeedValue::Float(node) => {
            visitor.visit_float_value(node, &path.join(segment, node))
        }
        ConstantPdaSeedValue::Injected(node) => {
            visitor.visit_injected_value(node, &path.join(segment, node))
        }
        ConstantPdaSeedValue::Integer(node) => {
            visitor.visit_integer_value(node, &path.join(segment, node))
        }
        ConstantPdaSeedValue::Map(node) => visitor.visit_map_value(node, &path.join(segment, node)),
        ConstantPdaSeedValue::None(node) => {
            visitor.visit_none_value(node, &path.join(segment, node))
        }
        ConstantPdaSeedValue::ProgramId(node) => {
            visitor.visit_program_id_value(node, &path.join(segment, node))
        }
        ConstantPdaSeedValue::PublicKey(node) => {
            visitor.visit_public_key_value(node, &path.join(segment, node))
        }
        ConstantPdaSeedValue::Set(node) => visitor.visit_set_value(node, &path.join(segment, node)),
        ConstantPdaSeedValue::Some(node) => {
            visitor.visit_some_value(node, &path.join(segment, node))
        }
        ConstantPdaSeedValue::String(node) => {
            visitor.visit_string_value(node, &path.join(segment, node))
        }
        ConstantPdaSeedValue::Struct(node) => {
            visitor.visit_struct_value(node, &path.join(segment, node))
        }
        ConstantPdaSeedValue::Tuple(node) => {
            visitor.visit_tuple_value(node, &path.join(segment, node))
        }
    }
}

pub fn walk_contextual_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a ContextualValueNode,
    path: &NodePath<'a>,
    segment: PathSegment,
) {
    match node {
        ContextualValueNode::Account(node) => {
            visitor.visit_account_value(node, &path.join_named(segment, node))
        }
        ContextualValueNode::AccountBump(node) => {
            visitor.visit_account_bump_value(node, &path.join_named(segment, node))
        }
        ContextualValueNode::AccountData(node) => {
            visitor.visit_account_data_value(node, &path.join(segment, node))
        }
        ContextualValueNode::Conditional(node) => {
            visitor.visit_conditional_value(node, &path.join(segment, node))
        }
        ContextualValueNode::Data(node) => {
            visitor.visit_data_value(node, &path.join(segment, node))
        }
        ContextualValueNode::Identity(node) => {
            visitor.visit_identity_value(node, &path.join(segment, node))
        }
        ContextualValueNode::Payer(node) => {
            visitor.visit_payer_value(node, &path.join(segment, node))
        }
        ContextualValueNode::Pda(node) => visitor.visit_pda_value(node, &path.join(segment, node)),
        ContextualValueNode::ProgramId(node) => {
            visitor.visit_program_id_value(node, &path.join(segment, node))
        }
    }
}

pub fn walk_count<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a CountNode,
    path: &NodePath<'a>,
    segment: PathSegment,
) {
    match node {
        CountNode::Fixed(node) => visitor.visit_fixed_count(node, &path.join(segment, node)),
        CountNode::Prefixed(node) => visitor.visit_prefixed_count(node, &path.join(segment, node)),
        CountNode::Remainder(node) => {
            visitor.visit_remainder_count(node, &path.join(segment, node))
        }
        CountNode::Sentinel(node) => visitor.visit_sentinel_count(node, &path.join(segment, node)),
    }
}

pub fn walk_discriminator<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a DiscriminatorNode,
    path: &NodePath<'a>,
    segment: PathSegment,
) {
    match node {
        DiscriminatorNode::Constant(node) => {
            visitor.visit_constant_discriminator(node, &path.join(segment, node))
        }
        DiscriminatorNode::Field(node) => {
            visitor.visit_field_discriminator(node, &path.join(segment, node))
        }
        DiscriminatorNode::Size(node) => {
            visitor.visit_size_discriminator(node, &path.join(segment, node))
        }
    }
}

pub fn walk_display<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a DisplayNode,
    path: &NodePath<'a>,
    segment: PathSegment,
) {
    match node {
        DisplayNode::AmountNumber(node) => {
            visitor.visit_amount_number_display(node, &path.join(segment, node))
        }
        DisplayNode::EnumVariant(node) => {
            visitor.visit_enum_variant_display(node, &path.join(segment, node))
        }
        DisplayNode::Instruction(node) => {
            visitor.visit_instruction_display(node, &path.join(segment, node))
        }
        DisplayNode::InstructionAccount(node) => {
            visitor.visit_instruction_account_display(node, &path.join(segment, node))
        }
        DisplayNode::String(node) => visitor.visit_string_display(node, &path.join(segment, node)),
        DisplayNode::StructField(node) => {
            visitor.visit_struct_field_display(node, &path.join(segment, node))
        }
        DisplayNode::UnitNumber(node) => {
            visitor.visit_unit_number_display(node, &path.join(segment, node))
        }
        DisplayNode::ValueLabel(node) => {
            visitor.visit_value_label_display(node, &path.join(segment, node))
        }
    }
}

pub fn walk_injectable_integer_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a InjectableIntegerValueNode,
    path: &NodePath<'a>,
    segment: PathSegment,
) {
    match node {
        InjectableIntegerValueNode::Injected(node) => {
            visitor.visit_injected_value(node, &path.join(segment, node))
        }
        InjectableIntegerValueNode::Integer(node) => {
            visitor.visit_integer_value(node, &path.join(segment, node))
        }
    }
}

pub fn walk_injectable_unit_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a InjectableUnitValueNode,
    path: &NodePath<'a>,
    segment: PathSegment,
) {
    match node {
        InjectableUnitValueNode::Injected(node) => {
            visitor.visit_injected_value(node, &path.join(segment, node))
        }
        InjectableUnitValueNode::PublicKey(node) => {
            visitor.visit_public_key_value(node, &path.join(segment, node))
        }
        InjectableUnitValueNode::String(node) => {
            visitor.visit_string_value(node, &path.join(segment, node))
        }
    }
}

pub fn walk_instruction_byte_delta_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a InstructionByteDeltaValue,
    path: &NodePath<'a>,
    segment: PathSegment,
) {
    match node {
        InstructionByteDeltaValue::AccountLink(node) => {
            visitor.visit_account_link(node, &path.join_named(segment, node))
        }
        InstructionByteDeltaValue::DataValue(node) => {
            visitor.visit_data_value(node, &path.join(segment, node))
        }
        InstructionByteDeltaValue::IntegerValue(node) => {
            visitor.visit_integer_value(node, &path.join(segment, node))
        }
    }
}

pub fn walk_instruction_input_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a InstructionInputValueNode,
    path: &NodePath<'a>,
    segment: PathSegment,
) {
    match node {
        InstructionInputValueNode::AccountBumpValue(node) => {
            visitor.visit_account_bump_value(node, &path.join_named(segment, node))
        }
        InstructionInputValueNode::AccountDataValue(node) => {
            visitor.visit_account_data_value(node, &path.join(segment, node))
        }
        InstructionInputValueNode::AccountValue(node) => {
            visitor.visit_account_value(node, &path.join_named(segment, node))
        }
        InstructionInputValueNode::ArrayValue(node) => {
            visitor.visit_array_value(node, &path.join(segment, node))
        }
        InstructionInputValueNode::BooleanValue(node) => {
            visitor.visit_boolean_value(node, &path.join(segment, node))
        }
        InstructionInputValueNode::BytesValue(node) => {
            visitor.visit_bytes_value(node, &path.join(segment, node))
        }
        InstructionInputValueNode::ConditionalValue(node) => {
            visitor.visit_conditional_value(node, &path.join(segment, node))
        }
        InstructionInputValueNode::ConstantValue(node) => {
            visitor.visit_constant_value(node, &path.join(segment, node))
        }
        InstructionInputValueNode::DataValue(node) => {
            visitor.visit_data_value(node, &path.join(segment, node))
        }
        InstructionInputValueNode::EnumValue(node) => {
            visitor.visit_enum_value(node, &path.join(segment, node))
        }
        InstructionInputValueNode::FloatValue(node) => {
            visitor.visit_float_value(node, &path.join(segment, node))
        }
        InstructionInputValueNode::IdentityValue(node) => {
            visitor.visit_identity_value(node, &path.join(segment, node))
        }
        InstructionInputValueNode::InjectedValue(node) => {
            visitor.visit_injected_value(node, &path.join(segment, node))
        }
        InstructionInputValueNode::IntegerValue(node) => {
            visitor.visit_integer_value(node, &path.join(segment, node))
        }
        InstructionInputValueNode::MapValue(node) => {
            visitor.visit_map_value(node, &path.join(segment, node))
        }
        InstructionInputValueNode::NoneValue(node) => {
            visitor.visit_none_value(node, &path.join(segment, node))
        }
        InstructionInputValueNode::PayerValue(node) => {
            visitor.visit_payer_value(node, &path.join(segment, node))
        }
        InstructionInputValueNode::PdaValue(node) => {
            visitor.visit_pda_value(node, &path.join(segment, node))
        }
        InstructionInputValueNode::ProgramIdValue(node) => {
            visitor.visit_program_id_value(node, &path.join(segment, node))
        }
        InstructionInputValueNode::ProgramLink(node) => {
            visitor.visit_program_link(node, &path.join_named(segment, node))
        }
        InstructionInputValueNode::PublicKeyValue(node) => {
            visitor.visit_public_key_value(node, &path.join(segment, node))
        }
        InstructionInputValueNode::SetValue(node) => {
            visitor.visit_set_value(node, &path.join(segment, node))
        }
        InstructionInputValueNode::SomeValue(node) => {
            visitor.visit_some_value(node, &path.join(segment, node))
        }
        InstructionInputValueNode::StringValue(node) => {
            visitor.visit_string_value(node, &path.join(segment, node))
        }
        InstructionInputValueNode::StructValue(node) => {
            visitor.visit_struct_value(node, &path.join(segment, node))
        }
        InstructionInputValueNode::TupleValue(node) => {
            visitor.visit_tuple_value(node, &path.join(segment, node))
        }
    }
}

pub fn walk_link<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a LinkNode,
    path: &NodePath<'a>,
    segment: PathSegment,
) {
    match node {
        LinkNode::Account(node) => {
            visitor.visit_account_link(node, &path.join_named(segment, node))
        }
        LinkNode::DefinedType(node) => {
            visitor.visit_defined_type_link(node, &path.join_named(segment, node))
        }
        LinkNode::Instruction(node) => {
            visitor.visit_instruction_link(node, &path.join_named(segment, node))
        }
        LinkNode::InstructionAccount(node) => {
            visitor.visit_instruction_account_link(node, &path.join_named(segment, node))
        }
        LinkNode::Pda(node) => visitor.visit_pda_link(node, &path.join_named(segment, node)),
        LinkNode::Program(node) => {
            visitor.visit_program_link(node, &path.join_named(segment, node))
        }
    }
}

pub fn walk_number_display<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a NumberDisplayNode,
    path: &NodePath<'a>,
    segment: PathSegment,
) {
    match node {
        NumberDisplayNode::Amount(node) => {
            visitor.visit_amount_number_display(node, &path.join(segment, node))
        }
        NumberDisplayNode::Unit(node) => {
            visitor.visit_unit_number_display(node, &path.join(segment, node))
        }
    }
}

pub fn walk_pda_seed<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a PdaSeedNode,
    path: &NodePath<'a>,
    segment: PathSegment,
) {
    match node {
        PdaSeedNode::Constant(node) => {
            visitor.visit_constant_pda_seed(node, &path.join(segment, node))
        }
        PdaSeedNode::Variable(node) => {
            visitor.visit_variable_pda_seed(node, &path.join_named(segment, node))
        }
    }
}

pub fn walk_pda_seed_value_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a PdaSeedValueValue,
    path: &NodePath<'a>,
    segment: PathSegment,
) {
    match node {
        PdaSeedValueValue::Account(node) => {
            visitor.visit_account_value(node, &path.join_named(segment, node))
        }
        PdaSeedValueValue::Array(node) => {
            visitor.visit_array_value(node, &path.join(segment, node))
        }
        PdaSeedValueValue::Boolean(node) => {
            visitor.visit_boolean_value(node, &path.join(segment, node))
        }
        PdaSeedValueValue::Bytes(node) => {
            visitor.visit_bytes_value(node, &path.join(segment, node))
        }
        PdaSeedValueValue::Constant(node) => {
            visitor.visit_constant_value(node, &path.join(segment, node))
        }
        PdaSeedValueValue::Data(node) => visitor.visit_data_value(node, &path.join(segment, node)),
        PdaSeedValueValue::Enum(node) => visitor.visit_enum_value(node, &path.join(segment, node)),
        PdaSeedValueValue::Float(node) => {
            visitor.visit_float_value(node, &path.join(segment, node))
        }
        PdaSeedValueValue::Injected(node) => {
            visitor.visit_injected_value(node, &path.join(segment, node))
        }
        PdaSeedValueValue::Integer(node) => {
            visitor.visit_integer_value(node, &path.join(segment, node))
        }
        PdaSeedValueValue::Map(node) => visitor.visit_map_value(node, &path.join(segment, node)),
        PdaSeedValueValue::None(node) => visitor.visit_none_value(node, &path.join(segment, node)),
        PdaSeedValueValue::PublicKey(node) => {
            visitor.visit_public_key_value(node, &path.join(segment, node))
        }
        PdaSeedValueValue::Set(node) => visitor.visit_set_value(node, &path.join(segment, node)),
        PdaSeedValueValue::Some(node) => visitor.visit_some_value(node, &path.join(segment, node)),
        PdaSeedValueValue::String(node) => {
            visitor.visit_string_value(node, &path.join(segment, node))
        }
        PdaSeedValueValue::Struct(node) => {
            visitor.visit_struct_value(node, &path.join(segment, node))
        }
        PdaSeedValueValue::Tuple(node) => {
            visitor.visit_tuple_value(node, &path.join(segment, node))
        }
    }
}

pub fn walk_pda_value_pda<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a PdaValuePda,
    path: &NodePath<'a>,
    segment: PathSegment,
) {
    match node {
        PdaValuePda::Pda(node) => visitor.visit_pda(node, &path.join_named(segment, node)),
        PdaValuePda::PdaLink(node) => visitor.visit_pda_link(node, &path.join_named(segment, node)),
    }
}

pub fn walk_pda_value_program_id<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a PdaValueProgramId,
    path: &NodePath<'a>,
    segment: PathSegment,
) {
    match node {
        PdaValueProgramId::Account(node) => {
            visitor.visit_account_value(node, &path.join_named(segment, node))
        }
        PdaValueProgramId::Data(node) => visitor.visit_data_value(node, &path.join(segment, node)),
    }
}

pub fn walk_registered_contextual_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a RegisteredContextualValueNode,
    path: &NodePath<'a>,
    segment: PathSegment,
) {
    match node {
        RegisteredContextualValueNode::Account(node) => {
            visitor.visit_account_value(node, &path.join_named(segment, node))
        }
        RegisteredContextualValueNode::AccountBump(node) => {
            visitor.visit_account_bump_value(node, &path.join_named(segment, node))
        }
        RegisteredContextualValueNode::AccountData(node) => {
            visitor.visit_account_data_value(node, &path.join(segment, node))
        }
        RegisteredContextualValueNode::Conditional(node) => {
            visitor.visit_conditional_value(node, &path.join(segment, node))
        }
        RegisteredContextualValueNode::Data(node) => {
            visitor.visit_data_value(node, &path.join(segment, node))
        }
        RegisteredContextualValueNode::Identity(node) => {
            visitor.visit_identity_value(node, &path.join(segment, node))
        }
        RegisteredContextualValueNode::Payer(node) => {
            visitor.visit_payer_value(node, &path.join(segment, node))
        }
        RegisteredContextualValueNode::Pda(node) => {
            visitor.visit_pda_value(node, &path.join(segment, node))
        }
        RegisteredContextualValueNode::ProgramId(node) => {
            visitor.visit_program_id_value(node, &path.join(segment, node))
        }
        RegisteredContextualValueNode::PdaSeed(node) => {
            visitor.visit_pda_seed_value(node, &path.join_named(segment, node))
        }
    }
}

pub fn walk_registered_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a RegisteredValueNode,
    path: &NodePath<'a>,
    segment: PathSegment,
) {
    match node {
        RegisteredValueNode::Array(node) => {
            visitor.visit_array_value(node, &path.join(segment, node))
        }
        RegisteredValueNode::Boolean(node) => {
            visitor.visit_boolean_value(node, &path.join(segment, node))
        }
        RegisteredValueNode::Bytes(node) => {
            visitor.visit_bytes_value(node, &path.join(segment, node))
        }
        RegisteredValueNode::Constant(node) => {
            visitor.visit_constant_value(node, &path.join(segment, node))
        }
        RegisteredValueNode::Enum(node) => {
            visitor.visit_enum_value(node, &path.join(segment, node))
        }
        RegisteredValueNode::Float(node) => {
            visitor.visit_float_value(node, &path.join(segment, node))
        }
        RegisteredValueNode::Injected(node) => {
            visitor.visit_injected_value(node, &path.join(segment, node))
        }
        RegisteredValueNode::Integer(node) => {
            visitor.visit_integer_value(node, &path.join(segment, node))
        }
        RegisteredValueNode::Map(node) => visitor.visit_map_value(node, &path.join(segment, node)),
        RegisteredValueNode::None(node) => {
            visitor.visit_none_value(node, &path.join(segment, node))
        }
        RegisteredValueNode::PublicKey(node) => {
            visitor.visit_public_key_value(node, &path.join(segment, node))
        }
        RegisteredValueNode::Set(node) => visitor.visit_set_value(node, &path.join(segment, node)),
        RegisteredValueNode::Some(node) => {
            visitor.visit_some_value(node, &path.join(segment, node))
        }
        RegisteredValueNode::String(node) => {
            visitor.visit_string_value(node, &path.join(segment, node))
        }
        RegisteredValueNode::Struct(node) => {
            visitor.visit_struct_value(node, &path.join(segment, node))
        }
        RegisteredValueNode::Tuple(node) => {
            visitor.visit_tuple_value(node, &path.join(segment, node))
        }
        RegisteredValueNode::MapEntry(node) => {
            visitor.visit_map_entry_value(node, &path.join(segment, node))
        }
        RegisteredValueNode::StructField(node) => {
            visitor.visit_struct_field_value(node, &path.join_named(segment, node))
        }
    }
}

pub fn walk_transform<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a TransformNode,
    path: &NodePath<'a>,
    segment: PathSegment,
) {
    match node {
        TransformNode::FixedSize(node) => {
            visitor.visit_fixed_size_transform(node, &path.join(segment, node))
        }
        TransformNode::HiddenPrefix(node) => {
            visitor.visit_hidden_prefix_transform(node, &path.join(segment, node))
        }
        TransformNode::HiddenSuffix(node) => {
            visitor.visit_hidden_suffix_transform(node, &path.join(segment, node))
        }
        TransformNode::PostOffset(node) => {
            visitor.visit_post_offset_transform(node, &path.join(segment, node))
        }
        TransformNode::PreOffset(node) => {
            visitor.visit_pre_offset_transform(node, &path.join(segment, node))
        }
        TransformNode::Sentinel(node) => {
            visitor.visit_sentinel_transform(node, &path.join(segment, node))
        }
        TransformNode::SizePrefix(node) => {
            visitor.visit_size_prefix_transform(node, &path.join(segment, node))
        }
    }
}

pub fn walk_value<'a, V: NodeVisitor + ?Sized>(
    visitor: &mut V,
    node: &'a ValueNode,
    path: &NodePath<'a>,
    segment: PathSegment,
) {
    match node {
        ValueNode::Array(node) => visitor.visit_array_value(node, &path.join(segment, node)),
        ValueNode::Boolean(node) => visitor.visit_boolean_value(node, &path.join(segment, node)),
        ValueNode::Bytes(node) => visitor.visit_bytes_value(node, &path.join(segment, node)),
        ValueNode::Constant(node) => visitor.visit_constant_value(node, &path.join(segment, node)),
        ValueNode::Enum(node) => visitor.visit_enum_value(node, &path.join(segment, node)),
        ValueNode::Float(node) => visitor.visit_float_value(node, &path.join(segment, node)),
        ValueNode::Injected(node) => visitor.visit_injected_value(node, &path.join(segment, node)),
        ValueNode::Integer(node) => visitor.visit_integer_value(node, &path.join(segment, node)),
        ValueNode::Map(node) => visitor.visit_map_value(node, &path.join(segment, node)),
        ValueNode::None(node) => visitor.visit_none_value(node, &path.join(segment, node)),
        ValueNode::PublicKey(node) => {
            visitor.visit_public_key_value(node, &path.join(segment, node))
        }
        ValueNode::Set(node) => visitor.visit_set_value(node, &path.join(segment, node)),
        ValueNode::Some(node) => visitor.visit_some_value(node, &path.join(segment, node)),
        ValueNode::String(node) => visitor.visit_string_value(node, &path.join(segment, node)),
        ValueNode::Struct(node) => visitor.visit_struct_value(node, &path.join(segment, node)),
        ValueNode::Tuple(node) => visitor.visit_tuple_value(node, &path.join(segment, node)),
    }
}
