use codama_nodes::*;

// ---------------------------------------------------------------------------
// Small ownership helpers used by the `fold_*` functions.
//
// They take a child container by value and return a rebuilt one, so a parent
// `fold_*` can write `node.child = map_*(node.child, |c| v.visit_*(c))` using
// the standard "reassign a moved field" pattern.
// ---------------------------------------------------------------------------

// The `Box<_>` parameters are intentional: the node fields these helpers
// operate on are themselves boxed, so taking the box by value lets call sites
// pass the field directly.
#[allow(clippy::boxed_local)]
fn map_box<T>(b: Box<T>, f: impl FnOnce(T) -> T) -> Box<T> {
    Box::new(f(*b))
}

#[allow(clippy::boxed_local)]
fn map_box_opt<T>(b: Box<Option<T>>, f: impl FnOnce(T) -> T) -> Box<Option<T>> {
    Box::new((*b).map(f))
}

fn map_opt<T>(o: Option<T>, f: impl FnOnce(T) -> T) -> Option<T> {
    o.map(f)
}

fn map_vec<T>(items: Vec<T>, f: impl FnMut(T) -> T) -> Vec<T> {
    items.into_iter().map(f).collect()
}

/// A visitor that consumes a Codama IDL node tree and returns a (possibly
/// rewritten) tree of the same shape.
///
/// This is the Rust analogue of the TypeScript `identityVisitor` extended via
/// `extendVisitor`: every method has a default body that rebuilds the node by
/// recursing into its children, so an implementor overrides only the node kinds
/// it wants to change. To recurse from inside an override (the equivalent of
/// the TS `next` hook), call the matching free `fold_*` function.
///
/// The traversal mirrors the upstream `mergeVisitor` child table and recurses
/// into every child of every node. The leaves of [`NestedTypeNode`] fields are
/// recursed via [`NestedTypeNodeTrait::map_nested_type_node`], which preserves
/// the wrapper chain while visiting the inner type.
///
/// One deliberate simplification in this first iteration: nodes are
/// transformed, never deleted (there is no `null` return as in the TS
/// `identityVisitor`); a deleting variant can be layered on later.
pub trait TransformVisitor {
    // ------------------------------------------------------------------
    // Union / dispatch entry points.
    // ------------------------------------------------------------------

    fn visit_type_node(&mut self, node: TypeNode) -> TypeNode {
        fold_type_node(self, node)
    }
    fn visit_value_node(&mut self, node: ValueNode) -> ValueNode {
        fold_value_node(self, node)
    }
    fn visit_count_node(&mut self, node: CountNode) -> CountNode {
        fold_count_node(self, node)
    }
    fn visit_discriminator_node(&mut self, node: DiscriminatorNode) -> DiscriminatorNode {
        fold_discriminator_node(self, node)
    }
    fn visit_pda_seed_node(&mut self, node: PdaSeedNode) -> PdaSeedNode {
        fold_pda_seed_node(self, node)
    }
    fn visit_enum_variant_type_node(&mut self, node: EnumVariantTypeNode) -> EnumVariantTypeNode {
        fold_enum_variant_type_node(self, node)
    }
    fn visit_enum_value_payload(&mut self, node: EnumValuePayload) -> EnumValuePayload {
        fold_enum_value_payload(self, node)
    }
    fn visit_constant_pda_seed_value(
        &mut self,
        node: ConstantPdaSeedValue,
    ) -> ConstantPdaSeedValue {
        fold_constant_pda_seed_value(self, node)
    }
    fn visit_pda_value_pda(&mut self, node: PdaValuePda) -> PdaValuePda {
        fold_pda_value_pda(self, node)
    }
    fn visit_pda_value_program_id(&mut self, node: PdaValueProgramId) -> PdaValueProgramId {
        fold_pda_value_program_id(self, node)
    }
    fn visit_pda_seed_value_value(&mut self, node: PdaSeedValueValue) -> PdaSeedValueValue {
        fold_pda_seed_value_value(self, node)
    }
    fn visit_instruction_byte_delta_value(
        &mut self,
        node: InstructionByteDeltaValue,
    ) -> InstructionByteDeltaValue {
        fold_instruction_byte_delta_value(self, node)
    }
    fn visit_instruction_remaining_accounts_value(
        &mut self,
        node: InstructionRemainingAccountsValue,
    ) -> InstructionRemainingAccountsValue {
        fold_instruction_remaining_accounts_value(self, node)
    }
    fn visit_conditional_value_condition(
        &mut self,
        node: ConditionalValueCondition,
    ) -> ConditionalValueCondition {
        fold_conditional_value_condition(self, node)
    }
    fn visit_resolver_dependency(&mut self, node: ResolverDependency) -> ResolverDependency {
        fold_resolver_dependency(self, node)
    }
    fn visit_instruction_input_value_node(
        &mut self,
        node: InstructionInputValueNode,
    ) -> InstructionInputValueNode {
        fold_instruction_input_value_node(self, node)
    }

    // ------------------------------------------------------------------
    // Type nodes.
    // ------------------------------------------------------------------

    fn visit_number_type(&mut self, node: NumberTypeNode) -> NumberTypeNode {
        node
    }
    fn visit_bytes_type(&mut self, node: BytesTypeNode) -> BytesTypeNode {
        node
    }
    fn visit_public_key_type(&mut self, node: PublicKeyTypeNode) -> PublicKeyTypeNode {
        node
    }
    fn visit_string_type(&mut self, node: StringTypeNode) -> StringTypeNode {
        node
    }
    fn visit_enum_empty_variant_type(
        &mut self,
        node: EnumEmptyVariantTypeNode,
    ) -> EnumEmptyVariantTypeNode {
        node
    }
    fn visit_amount_type(&mut self, node: AmountTypeNode) -> AmountTypeNode {
        fold_amount_type(self, node)
    }
    fn visit_array_type(&mut self, node: ArrayTypeNode) -> ArrayTypeNode {
        fold_array_type(self, node)
    }
    fn visit_boolean_type(&mut self, node: BooleanTypeNode) -> BooleanTypeNode {
        fold_boolean_type(self, node)
    }
    fn visit_date_time_type(&mut self, node: DateTimeTypeNode) -> DateTimeTypeNode {
        fold_date_time_type(self, node)
    }
    fn visit_enum_type(&mut self, node: EnumTypeNode) -> EnumTypeNode {
        fold_enum_type(self, node)
    }
    fn visit_fixed_size_type(
        &mut self,
        node: FixedSizeTypeNode<TypeNode>,
    ) -> FixedSizeTypeNode<TypeNode> {
        fold_fixed_size_type(self, node)
    }
    fn visit_hidden_prefix_type(
        &mut self,
        node: HiddenPrefixTypeNode<TypeNode>,
    ) -> HiddenPrefixTypeNode<TypeNode> {
        fold_hidden_prefix_type(self, node)
    }
    fn visit_hidden_suffix_type(
        &mut self,
        node: HiddenSuffixTypeNode<TypeNode>,
    ) -> HiddenSuffixTypeNode<TypeNode> {
        fold_hidden_suffix_type(self, node)
    }
    fn visit_map_type(&mut self, node: MapTypeNode) -> MapTypeNode {
        fold_map_type(self, node)
    }
    fn visit_option_type(&mut self, node: OptionTypeNode) -> OptionTypeNode {
        fold_option_type(self, node)
    }
    fn visit_post_offset_type(
        &mut self,
        node: PostOffsetTypeNode<TypeNode>,
    ) -> PostOffsetTypeNode<TypeNode> {
        fold_post_offset_type(self, node)
    }
    fn visit_pre_offset_type(
        &mut self,
        node: PreOffsetTypeNode<TypeNode>,
    ) -> PreOffsetTypeNode<TypeNode> {
        fold_pre_offset_type(self, node)
    }
    fn visit_remainder_option_type(
        &mut self,
        node: RemainderOptionTypeNode,
    ) -> RemainderOptionTypeNode {
        fold_remainder_option_type(self, node)
    }
    fn visit_sentinel_type(
        &mut self,
        node: SentinelTypeNode<TypeNode>,
    ) -> SentinelTypeNode<TypeNode> {
        fold_sentinel_type(self, node)
    }
    fn visit_set_type(&mut self, node: SetTypeNode) -> SetTypeNode {
        fold_set_type(self, node)
    }
    fn visit_size_prefix_type(
        &mut self,
        node: SizePrefixTypeNode<TypeNode>,
    ) -> SizePrefixTypeNode<TypeNode> {
        fold_size_prefix_type(self, node)
    }
    fn visit_sol_amount_type(&mut self, node: SolAmountTypeNode) -> SolAmountTypeNode {
        fold_sol_amount_type(self, node)
    }
    fn visit_struct_type(&mut self, node: StructTypeNode) -> StructTypeNode {
        fold_struct_type(self, node)
    }
    fn visit_struct_field_type(&mut self, node: StructFieldTypeNode) -> StructFieldTypeNode {
        fold_struct_field_type(self, node)
    }
    fn visit_tuple_type(&mut self, node: TupleTypeNode) -> TupleTypeNode {
        fold_tuple_type(self, node)
    }
    fn visit_zeroable_option_type(
        &mut self,
        node: ZeroableOptionTypeNode,
    ) -> ZeroableOptionTypeNode {
        fold_zeroable_option_type(self, node)
    }
    fn visit_enum_struct_variant_type(
        &mut self,
        node: EnumStructVariantTypeNode,
    ) -> EnumStructVariantTypeNode {
        fold_enum_struct_variant_type(self, node)
    }
    fn visit_enum_tuple_variant_type(
        &mut self,
        node: EnumTupleVariantTypeNode,
    ) -> EnumTupleVariantTypeNode {
        fold_enum_tuple_variant_type(self, node)
    }

    // ------------------------------------------------------------------
    // Value nodes.
    // ------------------------------------------------------------------

    fn visit_boolean_value(&mut self, node: BooleanValueNode) -> BooleanValueNode {
        node
    }
    fn visit_bytes_value(&mut self, node: BytesValueNode) -> BytesValueNode {
        node
    }
    fn visit_none_value(&mut self, node: NoneValueNode) -> NoneValueNode {
        node
    }
    fn visit_number_value(&mut self, node: NumberValueNode) -> NumberValueNode {
        node
    }
    fn visit_public_key_value(&mut self, node: PublicKeyValueNode) -> PublicKeyValueNode {
        node
    }
    fn visit_string_value(&mut self, node: StringValueNode) -> StringValueNode {
        node
    }
    fn visit_array_value(&mut self, node: ArrayValueNode) -> ArrayValueNode {
        fold_array_value(self, node)
    }
    fn visit_constant_value(&mut self, node: ConstantValueNode) -> ConstantValueNode {
        fold_constant_value(self, node)
    }
    fn visit_enum_value(&mut self, node: EnumValueNode) -> EnumValueNode {
        fold_enum_value(self, node)
    }
    fn visit_map_value(&mut self, node: MapValueNode) -> MapValueNode {
        fold_map_value(self, node)
    }
    fn visit_map_entry_value(&mut self, node: MapEntryValueNode) -> MapEntryValueNode {
        fold_map_entry_value(self, node)
    }
    fn visit_set_value(&mut self, node: SetValueNode) -> SetValueNode {
        fold_set_value(self, node)
    }
    fn visit_some_value(&mut self, node: SomeValueNode) -> SomeValueNode {
        fold_some_value(self, node)
    }
    fn visit_struct_value(&mut self, node: StructValueNode) -> StructValueNode {
        fold_struct_value(self, node)
    }
    fn visit_struct_field_value(&mut self, node: StructFieldValueNode) -> StructFieldValueNode {
        fold_struct_field_value(self, node)
    }
    fn visit_tuple_value(&mut self, node: TupleValueNode) -> TupleValueNode {
        fold_tuple_value(self, node)
    }

    // ------------------------------------------------------------------
    // Count nodes.
    // ------------------------------------------------------------------

    fn visit_fixed_count(&mut self, node: FixedCountNode) -> FixedCountNode {
        node
    }
    fn visit_remainder_count(&mut self, node: RemainderCountNode) -> RemainderCountNode {
        node
    }
    fn visit_prefixed_count(&mut self, node: PrefixedCountNode) -> PrefixedCountNode {
        fold_prefixed_count(self, node)
    }

    // ------------------------------------------------------------------
    // Discriminator nodes.
    // ------------------------------------------------------------------

    fn visit_field_discriminator(
        &mut self,
        node: FieldDiscriminatorNode,
    ) -> FieldDiscriminatorNode {
        node
    }
    fn visit_size_discriminator(&mut self, node: SizeDiscriminatorNode) -> SizeDiscriminatorNode {
        node
    }
    fn visit_constant_discriminator(
        &mut self,
        node: ConstantDiscriminatorNode,
    ) -> ConstantDiscriminatorNode {
        fold_constant_discriminator(self, node)
    }

    // ------------------------------------------------------------------
    // PDA seed nodes.
    // ------------------------------------------------------------------

    fn visit_constant_pda_seed(&mut self, node: ConstantPdaSeedNode) -> ConstantPdaSeedNode {
        fold_constant_pda_seed(self, node)
    }
    fn visit_variable_pda_seed(&mut self, node: VariablePdaSeedNode) -> VariablePdaSeedNode {
        fold_variable_pda_seed(self, node)
    }

    // ------------------------------------------------------------------
    // Link nodes.
    // ------------------------------------------------------------------

    fn visit_program_link(&mut self, node: ProgramLinkNode) -> ProgramLinkNode {
        node
    }
    fn visit_account_link(&mut self, node: AccountLinkNode) -> AccountLinkNode {
        fold_account_link(self, node)
    }
    fn visit_defined_type_link(&mut self, node: DefinedTypeLinkNode) -> DefinedTypeLinkNode {
        fold_defined_type_link(self, node)
    }
    fn visit_instruction_link(&mut self, node: InstructionLinkNode) -> InstructionLinkNode {
        fold_instruction_link(self, node)
    }
    fn visit_instruction_account_link(
        &mut self,
        node: InstructionAccountLinkNode,
    ) -> InstructionAccountLinkNode {
        fold_instruction_account_link(self, node)
    }
    fn visit_instruction_argument_link(
        &mut self,
        node: InstructionArgumentLinkNode,
    ) -> InstructionArgumentLinkNode {
        fold_instruction_argument_link(self, node)
    }
    fn visit_pda_link(&mut self, node: PdaLinkNode) -> PdaLinkNode {
        fold_pda_link(self, node)
    }

    // ------------------------------------------------------------------
    // Contextual value nodes.
    // ------------------------------------------------------------------

    fn visit_account_value(&mut self, node: AccountValueNode) -> AccountValueNode {
        node
    }
    fn visit_account_bump_value(&mut self, node: AccountBumpValueNode) -> AccountBumpValueNode {
        node
    }
    fn visit_argument_value(&mut self, node: ArgumentValueNode) -> ArgumentValueNode {
        node
    }
    fn visit_identity_value(&mut self, node: IdentityValueNode) -> IdentityValueNode {
        node
    }
    fn visit_payer_value(&mut self, node: PayerValueNode) -> PayerValueNode {
        node
    }
    fn visit_program_id_value(&mut self, node: ProgramIdValueNode) -> ProgramIdValueNode {
        node
    }
    fn visit_conditional_value(&mut self, node: ConditionalValueNode) -> ConditionalValueNode {
        fold_conditional_value(self, node)
    }
    fn visit_pda_value(&mut self, node: PdaValueNode) -> PdaValueNode {
        fold_pda_value(self, node)
    }
    fn visit_resolver_value(&mut self, node: ResolverValueNode) -> ResolverValueNode {
        fold_resolver_value(self, node)
    }
    fn visit_pda_seed_value(&mut self, node: PdaSeedValueNode) -> PdaSeedValueNode {
        fold_pda_seed_value(self, node)
    }

    // ------------------------------------------------------------------
    // Top-level nodes.
    // ------------------------------------------------------------------

    fn visit_error(&mut self, node: ErrorNode) -> ErrorNode {
        node
    }
    fn visit_instruction_status(&mut self, node: InstructionStatusNode) -> InstructionStatusNode {
        node
    }
    fn visit_account(&mut self, node: AccountNode) -> AccountNode {
        fold_account(self, node)
    }
    fn visit_constant(&mut self, node: ConstantNode) -> ConstantNode {
        fold_constant(self, node)
    }
    fn visit_defined_type(&mut self, node: DefinedTypeNode) -> DefinedTypeNode {
        fold_defined_type(self, node)
    }
    fn visit_event(&mut self, node: EventNode) -> EventNode {
        fold_event(self, node)
    }
    fn visit_instruction(&mut self, node: InstructionNode) -> InstructionNode {
        fold_instruction(self, node)
    }
    fn visit_instruction_account(
        &mut self,
        node: InstructionAccountNode,
    ) -> InstructionAccountNode {
        fold_instruction_account(self, node)
    }
    fn visit_instruction_argument(
        &mut self,
        node: InstructionArgumentNode,
    ) -> InstructionArgumentNode {
        fold_instruction_argument(self, node)
    }
    fn visit_instruction_byte_delta(
        &mut self,
        node: InstructionByteDeltaNode,
    ) -> InstructionByteDeltaNode {
        fold_instruction_byte_delta(self, node)
    }
    fn visit_instruction_remaining_accounts(
        &mut self,
        node: InstructionRemainingAccountsNode,
    ) -> InstructionRemainingAccountsNode {
        fold_instruction_remaining_accounts(self, node)
    }
    fn visit_pda(&mut self, node: PdaNode) -> PdaNode {
        fold_pda(self, node)
    }
    fn visit_program(&mut self, node: ProgramNode) -> ProgramNode {
        fold_program(self, node)
    }
    fn visit_root(&mut self, node: RootNode) -> RootNode {
        fold_root(self, node)
    }
}

// ===========================================================================
// Free `fold_*` functions: the default recursion for each node kind.
//
// Each rebuilds the node by visiting its children. Call these from inside a
// `visit_*` override to recurse into children after (or before) your own logic.
// ===========================================================================

// ---- Union dispatchers ----------------------------------------------------

pub fn fold_type_node<V: TransformVisitor + ?Sized>(v: &mut V, node: TypeNode) -> TypeNode {
    match node {
        TypeNode::Amount(n) => TypeNode::Amount(v.visit_amount_type(n)),
        TypeNode::Array(n) => TypeNode::Array(v.visit_array_type(n)),
        TypeNode::Boolean(n) => TypeNode::Boolean(v.visit_boolean_type(n)),
        TypeNode::Bytes(n) => TypeNode::Bytes(v.visit_bytes_type(n)),
        TypeNode::DateTime(n) => TypeNode::DateTime(v.visit_date_time_type(n)),
        TypeNode::Enum(n) => TypeNode::Enum(v.visit_enum_type(n)),
        TypeNode::FixedSize(n) => TypeNode::FixedSize(v.visit_fixed_size_type(n)),
        TypeNode::HiddenPrefix(n) => TypeNode::HiddenPrefix(v.visit_hidden_prefix_type(n)),
        TypeNode::HiddenSuffix(n) => TypeNode::HiddenSuffix(v.visit_hidden_suffix_type(n)),
        TypeNode::Map(n) => TypeNode::Map(v.visit_map_type(n)),
        TypeNode::Number(n) => TypeNode::Number(v.visit_number_type(n)),
        TypeNode::Option(n) => TypeNode::Option(v.visit_option_type(n)),
        TypeNode::PostOffset(n) => TypeNode::PostOffset(v.visit_post_offset_type(n)),
        TypeNode::PreOffset(n) => TypeNode::PreOffset(v.visit_pre_offset_type(n)),
        TypeNode::PublicKey(n) => TypeNode::PublicKey(v.visit_public_key_type(n)),
        TypeNode::RemainderOption(n) => TypeNode::RemainderOption(v.visit_remainder_option_type(n)),
        TypeNode::Sentinel(n) => TypeNode::Sentinel(v.visit_sentinel_type(n)),
        TypeNode::Set(n) => TypeNode::Set(v.visit_set_type(n)),
        TypeNode::SizePrefix(n) => TypeNode::SizePrefix(v.visit_size_prefix_type(n)),
        TypeNode::SolAmount(n) => TypeNode::SolAmount(v.visit_sol_amount_type(n)),
        TypeNode::String(n) => TypeNode::String(v.visit_string_type(n)),
        TypeNode::Struct(n) => TypeNode::Struct(v.visit_struct_type(n)),
        TypeNode::Tuple(n) => TypeNode::Tuple(v.visit_tuple_type(n)),
        TypeNode::ZeroableOption(n) => TypeNode::ZeroableOption(v.visit_zeroable_option_type(n)),
        TypeNode::Link(n) => TypeNode::Link(v.visit_defined_type_link(n)),
    }
}

pub fn fold_value_node<V: TransformVisitor + ?Sized>(v: &mut V, node: ValueNode) -> ValueNode {
    match node {
        ValueNode::Array(n) => ValueNode::Array(v.visit_array_value(n)),
        ValueNode::Boolean(n) => ValueNode::Boolean(v.visit_boolean_value(n)),
        ValueNode::Bytes(n) => ValueNode::Bytes(v.visit_bytes_value(n)),
        ValueNode::Constant(n) => ValueNode::Constant(v.visit_constant_value(n)),
        ValueNode::Enum(n) => ValueNode::Enum(v.visit_enum_value(n)),
        ValueNode::Map(n) => ValueNode::Map(v.visit_map_value(n)),
        ValueNode::None(n) => ValueNode::None(v.visit_none_value(n)),
        ValueNode::Number(n) => ValueNode::Number(v.visit_number_value(n)),
        ValueNode::PublicKey(n) => ValueNode::PublicKey(v.visit_public_key_value(n)),
        ValueNode::Set(n) => ValueNode::Set(v.visit_set_value(n)),
        ValueNode::Some(n) => ValueNode::Some(v.visit_some_value(n)),
        ValueNode::String(n) => ValueNode::String(v.visit_string_value(n)),
        ValueNode::Struct(n) => ValueNode::Struct(v.visit_struct_value(n)),
        ValueNode::Tuple(n) => ValueNode::Tuple(v.visit_tuple_value(n)),
    }
}

pub fn fold_count_node<V: TransformVisitor + ?Sized>(v: &mut V, node: CountNode) -> CountNode {
    match node {
        CountNode::Fixed(n) => CountNode::Fixed(v.visit_fixed_count(n)),
        CountNode::Prefixed(n) => CountNode::Prefixed(v.visit_prefixed_count(n)),
        CountNode::Remainder(n) => CountNode::Remainder(v.visit_remainder_count(n)),
    }
}

pub fn fold_discriminator_node<V: TransformVisitor + ?Sized>(
    v: &mut V,
    node: DiscriminatorNode,
) -> DiscriminatorNode {
    match node {
        DiscriminatorNode::Constant(n) => {
            DiscriminatorNode::Constant(v.visit_constant_discriminator(n))
        }
        DiscriminatorNode::Field(n) => DiscriminatorNode::Field(v.visit_field_discriminator(n)),
        DiscriminatorNode::Size(n) => DiscriminatorNode::Size(v.visit_size_discriminator(n)),
    }
}

pub fn fold_pda_seed_node<V: TransformVisitor + ?Sized>(
    v: &mut V,
    node: PdaSeedNode,
) -> PdaSeedNode {
    match node {
        PdaSeedNode::Constant(n) => PdaSeedNode::Constant(v.visit_constant_pda_seed(n)),
        PdaSeedNode::Variable(n) => PdaSeedNode::Variable(v.visit_variable_pda_seed(n)),
    }
}

pub fn fold_enum_variant_type_node<V: TransformVisitor + ?Sized>(
    v: &mut V,
    node: EnumVariantTypeNode,
) -> EnumVariantTypeNode {
    match node {
        EnumVariantTypeNode::Empty(n) => {
            EnumVariantTypeNode::Empty(v.visit_enum_empty_variant_type(n))
        }
        EnumVariantTypeNode::Struct(n) => {
            EnumVariantTypeNode::Struct(v.visit_enum_struct_variant_type(n))
        }
        EnumVariantTypeNode::Tuple(n) => {
            EnumVariantTypeNode::Tuple(v.visit_enum_tuple_variant_type(n))
        }
    }
}

pub fn fold_enum_value_payload<V: TransformVisitor + ?Sized>(
    v: &mut V,
    node: EnumValuePayload,
) -> EnumValuePayload {
    match node {
        EnumValuePayload::Struct(n) => EnumValuePayload::Struct(v.visit_struct_value(n)),
        EnumValuePayload::Tuple(n) => EnumValuePayload::Tuple(v.visit_tuple_value(n)),
    }
}

pub fn fold_constant_pda_seed_value<V: TransformVisitor + ?Sized>(
    v: &mut V,
    node: ConstantPdaSeedValue,
) -> ConstantPdaSeedValue {
    match node {
        ConstantPdaSeedValue::Array(n) => ConstantPdaSeedValue::Array(v.visit_array_value(n)),
        ConstantPdaSeedValue::Boolean(n) => ConstantPdaSeedValue::Boolean(v.visit_boolean_value(n)),
        ConstantPdaSeedValue::Bytes(n) => ConstantPdaSeedValue::Bytes(v.visit_bytes_value(n)),
        ConstantPdaSeedValue::Constant(n) => {
            ConstantPdaSeedValue::Constant(v.visit_constant_value(n))
        }
        ConstantPdaSeedValue::Enum(n) => ConstantPdaSeedValue::Enum(v.visit_enum_value(n)),
        ConstantPdaSeedValue::Map(n) => ConstantPdaSeedValue::Map(v.visit_map_value(n)),
        ConstantPdaSeedValue::None(n) => ConstantPdaSeedValue::None(v.visit_none_value(n)),
        ConstantPdaSeedValue::Number(n) => ConstantPdaSeedValue::Number(v.visit_number_value(n)),
        ConstantPdaSeedValue::ProgramId(n) => {
            ConstantPdaSeedValue::ProgramId(v.visit_program_id_value(n))
        }
        ConstantPdaSeedValue::PublicKey(n) => {
            ConstantPdaSeedValue::PublicKey(v.visit_public_key_value(n))
        }
        ConstantPdaSeedValue::Set(n) => ConstantPdaSeedValue::Set(v.visit_set_value(n)),
        ConstantPdaSeedValue::Some(n) => ConstantPdaSeedValue::Some(v.visit_some_value(n)),
        ConstantPdaSeedValue::String(n) => ConstantPdaSeedValue::String(v.visit_string_value(n)),
        ConstantPdaSeedValue::Struct(n) => ConstantPdaSeedValue::Struct(v.visit_struct_value(n)),
        ConstantPdaSeedValue::Tuple(n) => ConstantPdaSeedValue::Tuple(v.visit_tuple_value(n)),
    }
}

pub fn fold_pda_value_pda<V: TransformVisitor + ?Sized>(
    v: &mut V,
    node: PdaValuePda,
) -> PdaValuePda {
    match node {
        PdaValuePda::Pda(n) => PdaValuePda::Pda(v.visit_pda(n)),
        PdaValuePda::PdaLink(n) => PdaValuePda::PdaLink(v.visit_pda_link(n)),
    }
}

pub fn fold_pda_value_program_id<V: TransformVisitor + ?Sized>(
    v: &mut V,
    node: PdaValueProgramId,
) -> PdaValueProgramId {
    match node {
        PdaValueProgramId::Account(n) => PdaValueProgramId::Account(v.visit_account_value(n)),
        PdaValueProgramId::Argument(n) => PdaValueProgramId::Argument(v.visit_argument_value(n)),
    }
}

pub fn fold_pda_seed_value_value<V: TransformVisitor + ?Sized>(
    v: &mut V,
    node: PdaSeedValueValue,
) -> PdaSeedValueValue {
    match node {
        PdaSeedValueValue::Account(n) => PdaSeedValueValue::Account(v.visit_account_value(n)),
        PdaSeedValueValue::Argument(n) => PdaSeedValueValue::Argument(v.visit_argument_value(n)),
        PdaSeedValueValue::Array(n) => PdaSeedValueValue::Array(v.visit_array_value(n)),
        PdaSeedValueValue::Boolean(n) => PdaSeedValueValue::Boolean(v.visit_boolean_value(n)),
        PdaSeedValueValue::Bytes(n) => PdaSeedValueValue::Bytes(v.visit_bytes_value(n)),
        PdaSeedValueValue::Constant(n) => PdaSeedValueValue::Constant(v.visit_constant_value(n)),
        PdaSeedValueValue::Enum(n) => PdaSeedValueValue::Enum(v.visit_enum_value(n)),
        PdaSeedValueValue::Map(n) => PdaSeedValueValue::Map(v.visit_map_value(n)),
        PdaSeedValueValue::None(n) => PdaSeedValueValue::None(v.visit_none_value(n)),
        PdaSeedValueValue::Number(n) => PdaSeedValueValue::Number(v.visit_number_value(n)),
        PdaSeedValueValue::PublicKey(n) => {
            PdaSeedValueValue::PublicKey(v.visit_public_key_value(n))
        }
        PdaSeedValueValue::Set(n) => PdaSeedValueValue::Set(v.visit_set_value(n)),
        PdaSeedValueValue::Some(n) => PdaSeedValueValue::Some(v.visit_some_value(n)),
        PdaSeedValueValue::String(n) => PdaSeedValueValue::String(v.visit_string_value(n)),
        PdaSeedValueValue::Struct(n) => PdaSeedValueValue::Struct(v.visit_struct_value(n)),
        PdaSeedValueValue::Tuple(n) => PdaSeedValueValue::Tuple(v.visit_tuple_value(n)),
    }
}

pub fn fold_instruction_byte_delta_value<V: TransformVisitor + ?Sized>(
    v: &mut V,
    node: InstructionByteDeltaValue,
) -> InstructionByteDeltaValue {
    match node {
        InstructionByteDeltaValue::AccountLink(n) => {
            InstructionByteDeltaValue::AccountLink(v.visit_account_link(n))
        }
        InstructionByteDeltaValue::ArgumentValue(n) => {
            InstructionByteDeltaValue::ArgumentValue(v.visit_argument_value(n))
        }
        InstructionByteDeltaValue::NumberValue(n) => {
            InstructionByteDeltaValue::NumberValue(v.visit_number_value(n))
        }
        InstructionByteDeltaValue::ResolverValue(n) => {
            InstructionByteDeltaValue::ResolverValue(v.visit_resolver_value(n))
        }
    }
}

pub fn fold_instruction_remaining_accounts_value<V: TransformVisitor + ?Sized>(
    v: &mut V,
    node: InstructionRemainingAccountsValue,
) -> InstructionRemainingAccountsValue {
    match node {
        InstructionRemainingAccountsValue::Argument(n) => {
            InstructionRemainingAccountsValue::Argument(v.visit_argument_value(n))
        }
        InstructionRemainingAccountsValue::Resolver(n) => {
            InstructionRemainingAccountsValue::Resolver(v.visit_resolver_value(n))
        }
    }
}

pub fn fold_conditional_value_condition<V: TransformVisitor + ?Sized>(
    v: &mut V,
    node: ConditionalValueCondition,
) -> ConditionalValueCondition {
    match node {
        ConditionalValueCondition::Account(n) => {
            ConditionalValueCondition::Account(v.visit_account_value(n))
        }
        ConditionalValueCondition::Argument(n) => {
            ConditionalValueCondition::Argument(v.visit_argument_value(n))
        }
        ConditionalValueCondition::Resolver(n) => {
            ConditionalValueCondition::Resolver(v.visit_resolver_value(n))
        }
    }
}

pub fn fold_resolver_dependency<V: TransformVisitor + ?Sized>(
    v: &mut V,
    node: ResolverDependency,
) -> ResolverDependency {
    match node {
        ResolverDependency::Account(n) => ResolverDependency::Account(v.visit_account_value(n)),
        ResolverDependency::Argument(n) => ResolverDependency::Argument(v.visit_argument_value(n)),
    }
}

pub fn fold_instruction_input_value_node<V: TransformVisitor + ?Sized>(
    v: &mut V,
    node: InstructionInputValueNode,
) -> InstructionInputValueNode {
    use InstructionInputValueNode as N;
    match node {
        N::AccountBumpValue(n) => N::AccountBumpValue(v.visit_account_bump_value(n)),
        N::AccountValue(n) => N::AccountValue(v.visit_account_value(n)),
        N::ArgumentValue(n) => N::ArgumentValue(v.visit_argument_value(n)),
        N::ArrayValue(n) => N::ArrayValue(v.visit_array_value(n)),
        N::BooleanValue(n) => N::BooleanValue(v.visit_boolean_value(n)),
        N::BytesValue(n) => N::BytesValue(v.visit_bytes_value(n)),
        N::ConditionalValue(n) => N::ConditionalValue(v.visit_conditional_value(n)),
        N::ConstantValue(n) => N::ConstantValue(v.visit_constant_value(n)),
        N::EnumValue(n) => N::EnumValue(v.visit_enum_value(n)),
        N::IdentityValue(n) => N::IdentityValue(v.visit_identity_value(n)),
        N::MapValue(n) => N::MapValue(v.visit_map_value(n)),
        N::NoneValue(n) => N::NoneValue(v.visit_none_value(n)),
        N::NumberValue(n) => N::NumberValue(v.visit_number_value(n)),
        N::PayerValue(n) => N::PayerValue(v.visit_payer_value(n)),
        N::PdaValue(n) => N::PdaValue(v.visit_pda_value(n)),
        N::ProgramIdValue(n) => N::ProgramIdValue(v.visit_program_id_value(n)),
        N::ProgramLink(n) => N::ProgramLink(v.visit_program_link(n)),
        N::PublicKeyValue(n) => N::PublicKeyValue(v.visit_public_key_value(n)),
        N::ResolverValue(n) => N::ResolverValue(v.visit_resolver_value(n)),
        N::SetValue(n) => N::SetValue(v.visit_set_value(n)),
        N::SomeValue(n) => N::SomeValue(v.visit_some_value(n)),
        N::StringValue(n) => N::StringValue(v.visit_string_value(n)),
        N::StructValue(n) => N::StructValue(v.visit_struct_value(n)),
        N::TupleValue(n) => N::TupleValue(v.visit_tuple_value(n)),
    }
}

// ---- Type nodes -----------------------------------------------------------

pub fn fold_amount_type<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: AmountTypeNode,
) -> AmountTypeNode {
    node.number = node.number.map_nested_type_node(|n| v.visit_number_type(n));
    node
}

pub fn fold_array_type<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: ArrayTypeNode,
) -> ArrayTypeNode {
    node.item = map_box(node.item, |t| v.visit_type_node(t));
    node.count = map_box(node.count, |c| v.visit_count_node(c));
    node
}

pub fn fold_boolean_type<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: BooleanTypeNode,
) -> BooleanTypeNode {
    node.size = node.size.map_nested_type_node(|n| v.visit_number_type(n));
    node
}

pub fn fold_date_time_type<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: DateTimeTypeNode,
) -> DateTimeTypeNode {
    node.number = node.number.map_nested_type_node(|n| v.visit_number_type(n));
    node
}

pub fn fold_enum_type<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: EnumTypeNode,
) -> EnumTypeNode {
    node.variants = map_vec(node.variants, |variant| {
        v.visit_enum_variant_type_node(variant)
    });
    node.size = node.size.map_nested_type_node(|n| v.visit_number_type(n));
    node
}

pub fn fold_fixed_size_type<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: FixedSizeTypeNode<TypeNode>,
) -> FixedSizeTypeNode<TypeNode> {
    node.r#type = map_box(node.r#type, |t| v.visit_type_node(t));
    node
}

pub fn fold_hidden_prefix_type<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: HiddenPrefixTypeNode<TypeNode>,
) -> HiddenPrefixTypeNode<TypeNode> {
    node.prefix = map_vec(node.prefix, |c| v.visit_constant_value(c));
    node.r#type = map_box(node.r#type, |t| v.visit_type_node(t));
    node
}

pub fn fold_hidden_suffix_type<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: HiddenSuffixTypeNode<TypeNode>,
) -> HiddenSuffixTypeNode<TypeNode> {
    node.r#type = map_box(node.r#type, |t| v.visit_type_node(t));
    node.suffix = map_vec(node.suffix, |c| v.visit_constant_value(c));
    node
}

pub fn fold_map_type<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: MapTypeNode,
) -> MapTypeNode {
    node.key = map_box(node.key, |t| v.visit_type_node(t));
    node.value = map_box(node.value, |t| v.visit_type_node(t));
    node.count = map_box(node.count, |c| v.visit_count_node(c));
    node
}

pub fn fold_option_type<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: OptionTypeNode,
) -> OptionTypeNode {
    node.item = map_box(node.item, |t| v.visit_type_node(t));
    node.prefix = node.prefix.map_nested_type_node(|n| v.visit_number_type(n));
    node
}

pub fn fold_post_offset_type<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: PostOffsetTypeNode<TypeNode>,
) -> PostOffsetTypeNode<TypeNode> {
    node.r#type = map_box(node.r#type, |t| v.visit_type_node(t));
    node
}

pub fn fold_pre_offset_type<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: PreOffsetTypeNode<TypeNode>,
) -> PreOffsetTypeNode<TypeNode> {
    node.r#type = map_box(node.r#type, |t| v.visit_type_node(t));
    node
}

pub fn fold_remainder_option_type<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: RemainderOptionTypeNode,
) -> RemainderOptionTypeNode {
    node.item = map_box(node.item, |t| v.visit_type_node(t));
    node
}

pub fn fold_sentinel_type<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: SentinelTypeNode<TypeNode>,
) -> SentinelTypeNode<TypeNode> {
    node.r#type = map_box(node.r#type, |t| v.visit_type_node(t));
    node.sentinel = v.visit_constant_value(node.sentinel);
    node
}

pub fn fold_set_type<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: SetTypeNode,
) -> SetTypeNode {
    node.item = map_box(node.item, |t| v.visit_type_node(t));
    node.count = map_box(node.count, |c| v.visit_count_node(c));
    node
}

pub fn fold_size_prefix_type<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: SizePrefixTypeNode<TypeNode>,
) -> SizePrefixTypeNode<TypeNode> {
    node.prefix = Box::new((*node.prefix).map_nested_type_node(|n| v.visit_number_type(n)));
    node.r#type = map_box(node.r#type, |t| v.visit_type_node(t));
    node
}

pub fn fold_sol_amount_type<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: SolAmountTypeNode,
) -> SolAmountTypeNode {
    node.number = node.number.map_nested_type_node(|n| v.visit_number_type(n));
    node
}

pub fn fold_struct_type<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: StructTypeNode,
) -> StructTypeNode {
    node.fields = map_vec(node.fields, |field| v.visit_struct_field_type(field));
    node
}

pub fn fold_struct_field_type<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: StructFieldTypeNode,
) -> StructFieldTypeNode {
    node.r#type = map_box(node.r#type, |t| v.visit_type_node(t));
    node.default_value = map_box_opt(node.default_value, |val| v.visit_value_node(val));
    node
}

pub fn fold_tuple_type<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: TupleTypeNode,
) -> TupleTypeNode {
    node.items = map_vec(node.items, |item| v.visit_type_node(item));
    node
}

pub fn fold_zeroable_option_type<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: ZeroableOptionTypeNode,
) -> ZeroableOptionTypeNode {
    node.item = map_box(node.item, |t| v.visit_type_node(t));
    node.zero_value = map_opt(node.zero_value, |c| v.visit_constant_value(c));
    node
}

pub fn fold_enum_struct_variant_type<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: EnumStructVariantTypeNode,
) -> EnumStructVariantTypeNode {
    node.r#struct = node
        .r#struct
        .map_nested_type_node(|s| v.visit_struct_type(s));
    node
}

pub fn fold_enum_tuple_variant_type<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: EnumTupleVariantTypeNode,
) -> EnumTupleVariantTypeNode {
    node.tuple = node.tuple.map_nested_type_node(|t| v.visit_tuple_type(t));
    node
}

// ---- Value nodes ----------------------------------------------------------

pub fn fold_array_value<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: ArrayValueNode,
) -> ArrayValueNode {
    node.items = map_vec(node.items, |item| v.visit_value_node(item));
    node
}

pub fn fold_constant_value<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: ConstantValueNode,
) -> ConstantValueNode {
    node.r#type = map_box(node.r#type, |t| v.visit_type_node(t));
    node.value = map_box(node.value, |val| v.visit_value_node(val));
    node
}

pub fn fold_enum_value<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: EnumValueNode,
) -> EnumValueNode {
    node.r#enum = v.visit_defined_type_link(node.r#enum);
    node.value = map_box_opt(node.value, |p| v.visit_enum_value_payload(p));
    node
}

pub fn fold_map_value<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: MapValueNode,
) -> MapValueNode {
    node.entries = map_vec(node.entries, |entry| v.visit_map_entry_value(entry));
    node
}

pub fn fold_map_entry_value<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: MapEntryValueNode,
) -> MapEntryValueNode {
    node.key = map_box(node.key, |val| v.visit_value_node(val));
    node.value = map_box(node.value, |val| v.visit_value_node(val));
    node
}

pub fn fold_set_value<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: SetValueNode,
) -> SetValueNode {
    node.items = map_vec(node.items, |item| v.visit_value_node(item));
    node
}

pub fn fold_some_value<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: SomeValueNode,
) -> SomeValueNode {
    node.value = map_box(node.value, |val| v.visit_value_node(val));
    node
}

pub fn fold_struct_value<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: StructValueNode,
) -> StructValueNode {
    node.fields = map_vec(node.fields, |field| v.visit_struct_field_value(field));
    node
}

pub fn fold_struct_field_value<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: StructFieldValueNode,
) -> StructFieldValueNode {
    node.value = map_box(node.value, |val| v.visit_value_node(val));
    node
}

pub fn fold_tuple_value<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: TupleValueNode,
) -> TupleValueNode {
    node.items = map_vec(node.items, |item| v.visit_value_node(item));
    node
}

// ---- Count nodes ----------------------------------------------------------

pub fn fold_prefixed_count<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: PrefixedCountNode,
) -> PrefixedCountNode {
    node.prefix = node.prefix.map_nested_type_node(|n| v.visit_number_type(n));
    node
}

// ---- Discriminator nodes --------------------------------------------------

pub fn fold_constant_discriminator<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: ConstantDiscriminatorNode,
) -> ConstantDiscriminatorNode {
    node.constant = v.visit_constant_value(node.constant);
    node
}

// ---- PDA seed nodes -------------------------------------------------------

pub fn fold_constant_pda_seed<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: ConstantPdaSeedNode,
) -> ConstantPdaSeedNode {
    node.r#type = map_box(node.r#type, |t| v.visit_type_node(t));
    node.value = map_box(node.value, |val| v.visit_constant_pda_seed_value(val));
    node
}

pub fn fold_variable_pda_seed<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: VariablePdaSeedNode,
) -> VariablePdaSeedNode {
    node.r#type = map_box(node.r#type, |t| v.visit_type_node(t));
    node
}

// ---- Link nodes -----------------------------------------------------------

pub fn fold_account_link<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: AccountLinkNode,
) -> AccountLinkNode {
    node.program = map_opt(node.program, |p| v.visit_program_link(p));
    node
}

pub fn fold_defined_type_link<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: DefinedTypeLinkNode,
) -> DefinedTypeLinkNode {
    node.program = map_opt(node.program, |p| v.visit_program_link(p));
    node
}

pub fn fold_instruction_link<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: InstructionLinkNode,
) -> InstructionLinkNode {
    node.program = map_opt(node.program, |p| v.visit_program_link(p));
    node
}

pub fn fold_instruction_account_link<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: InstructionAccountLinkNode,
) -> InstructionAccountLinkNode {
    node.instruction = map_opt(node.instruction, |i| v.visit_instruction_link(i));
    node
}

pub fn fold_instruction_argument_link<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: InstructionArgumentLinkNode,
) -> InstructionArgumentLinkNode {
    node.instruction = map_opt(node.instruction, |i| v.visit_instruction_link(i));
    node
}

pub fn fold_pda_link<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: PdaLinkNode,
) -> PdaLinkNode {
    node.program = map_opt(node.program, |p| v.visit_program_link(p));
    node
}

// ---- Contextual value nodes -----------------------------------------------

pub fn fold_conditional_value<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: ConditionalValueNode,
) -> ConditionalValueNode {
    node.condition = map_box(node.condition, |c| v.visit_conditional_value_condition(c));
    node.value = map_box_opt(node.value, |val| v.visit_value_node(val));
    node.if_true = map_box_opt(node.if_true, |i| v.visit_instruction_input_value_node(i));
    node.if_false = map_box_opt(node.if_false, |i| v.visit_instruction_input_value_node(i));
    node
}

pub fn fold_pda_value<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: PdaValueNode,
) -> PdaValueNode {
    node.pda = map_box(node.pda, |p| v.visit_pda_value_pda(p));
    node.seeds = map_vec(node.seeds, |seed| v.visit_pda_seed_value(seed));
    node.program_id = map_box_opt(node.program_id, |p| v.visit_pda_value_program_id(p));
    node
}

pub fn fold_resolver_value<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: ResolverValueNode,
) -> ResolverValueNode {
    node.depends_on = map_vec(node.depends_on, |dep| v.visit_resolver_dependency(dep));
    node
}

pub fn fold_pda_seed_value<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: PdaSeedValueNode,
) -> PdaSeedValueNode {
    node.value = map_box(node.value, |val| v.visit_pda_seed_value_value(val));
    node
}

// ---- Top-level nodes ------------------------------------------------------

pub fn fold_account<V: TransformVisitor + ?Sized>(v: &mut V, mut node: AccountNode) -> AccountNode {
    node.data = node.data.map_nested_type_node(|s| v.visit_struct_type(s));
    node.pda = map_opt(node.pda, |p| v.visit_pda_link(p));
    node.discriminators = map_vec(node.discriminators, |d| v.visit_discriminator_node(d));
    node
}

pub fn fold_constant<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: ConstantNode,
) -> ConstantNode {
    node.r#type = map_box(node.r#type, |t| v.visit_type_node(t));
    node.value = map_box(node.value, |val| v.visit_value_node(val));
    node
}

pub fn fold_defined_type<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: DefinedTypeNode,
) -> DefinedTypeNode {
    node.r#type = map_box(node.r#type, |t| v.visit_type_node(t));
    node
}

pub fn fold_event<V: TransformVisitor + ?Sized>(v: &mut V, mut node: EventNode) -> EventNode {
    node.data = map_box(node.data, |t| v.visit_type_node(t));
    node.discriminators = map_vec(node.discriminators, |d| v.visit_discriminator_node(d));
    node
}

pub fn fold_instruction<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: InstructionNode,
) -> InstructionNode {
    node.accounts = map_vec(node.accounts, |a| v.visit_instruction_account(a));
    node.arguments = map_vec(node.arguments, |a| v.visit_instruction_argument(a));
    node.extra_arguments = map_vec(node.extra_arguments, |a| v.visit_instruction_argument(a));
    node.remaining_accounts = map_vec(node.remaining_accounts, |a| {
        v.visit_instruction_remaining_accounts(a)
    });
    node.byte_deltas = map_vec(node.byte_deltas, |b| v.visit_instruction_byte_delta(b));
    node.discriminators = map_vec(node.discriminators, |d| v.visit_discriminator_node(d));
    node.status = map_opt(node.status, |s| v.visit_instruction_status(s));
    node.sub_instructions = map_vec(node.sub_instructions, |i| v.visit_instruction(i));
    node
}

pub fn fold_instruction_account<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: InstructionAccountNode,
) -> InstructionAccountNode {
    node.default_value = map_box_opt(node.default_value, |i| {
        v.visit_instruction_input_value_node(i)
    });
    node
}

pub fn fold_instruction_argument<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: InstructionArgumentNode,
) -> InstructionArgumentNode {
    node.r#type = map_box(node.r#type, |t| v.visit_type_node(t));
    node.default_value = map_box_opt(node.default_value, |i| {
        v.visit_instruction_input_value_node(i)
    });
    node
}

pub fn fold_instruction_byte_delta<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: InstructionByteDeltaNode,
) -> InstructionByteDeltaNode {
    node.value = map_box(node.value, |val| v.visit_instruction_byte_delta_value(val));
    node
}

pub fn fold_instruction_remaining_accounts<V: TransformVisitor + ?Sized>(
    v: &mut V,
    mut node: InstructionRemainingAccountsNode,
) -> InstructionRemainingAccountsNode {
    node.value = map_box(node.value, |val| {
        v.visit_instruction_remaining_accounts_value(val)
    });
    node
}

pub fn fold_pda<V: TransformVisitor + ?Sized>(v: &mut V, mut node: PdaNode) -> PdaNode {
    node.seeds = map_vec(node.seeds, |seed| v.visit_pda_seed_node(seed));
    node
}

pub fn fold_program<V: TransformVisitor + ?Sized>(v: &mut V, mut node: ProgramNode) -> ProgramNode {
    node.pdas = map_vec(node.pdas, |p| v.visit_pda(p));
    node.accounts = map_vec(node.accounts, |a| v.visit_account(a));
    node.events = map_vec(node.events, |e| v.visit_event(e));
    node.instructions = map_vec(node.instructions, |i| v.visit_instruction(i));
    node.defined_types = map_vec(node.defined_types, |d| v.visit_defined_type(d));
    node.errors = map_vec(node.errors, |e| v.visit_error(e));
    node.constants = map_vec(node.constants, |c| v.visit_constant(c));
    node
}

pub fn fold_root<V: TransformVisitor + ?Sized>(v: &mut V, mut node: RootNode) -> RootNode {
    node.program = v.visit_program(node.program);
    node.additional_programs = map_vec(node.additional_programs, |p| v.visit_program(p));
    node
}
