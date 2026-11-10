use crate::node_ref::{
    map_array, map_docs, map_optional, map_optional_text, map_required, map_text, MapChild,
};
use crate::{
    AccountBumpValueNode, AccountDataValueNode, AccountLinkNode, AccountNode, AccountValueNode,
    AmountNumberDisplayNode, ArrayTypeNode, ArrayValueNode, BooleanTypeNode, BooleanValueNode,
    BytesTypeNode, BytesValueNode, ConditionalValueCondition, ConditionalValueNode,
    ConstantDiscriminatorNode, ConstantNode, ConstantPdaSeedNode, ConstantPdaSeedValue,
    ConstantValueNode, CountNode, DataValueNode, DateTimeTypeNode, DefinedTypeLinkNode,
    DefinedTypeNode, DiscriminatorNode, DurationTypeNode, EnumTypeNode, EnumValueNode,
    EnumVariantDisplayNode, EnumVariantTypeNode, ErrorNode, EventNode, FieldDiscriminatorNode,
    FixedCountNode, FixedPointTypeNode, FixedSizeTransformNode, FloatTypeNode, FloatValueNode,
    HasKind, HiddenPrefixTransformNode, HiddenSuffixTransformNode, Identifier, IdentityValueNode,
    InjectableIntegerValueNode, InjectableUnitValueNode, InjectedValueNode,
    InstructionAccountDisplayNode, InstructionAccountLinkNode, InstructionAccountNode,
    InstructionByteDeltaNode, InstructionByteDeltaValue, InstructionDisplayNode,
    InstructionInputValueNode, InstructionLinkNode, InstructionNode,
    InstructionRemainingAccountsNode, InstructionStatusNode, IntegerTypeNode, IntegerValueNode,
    MapEntryValueNode, MapTypeNode, MapValueNode, NoneValueNode, NumberDisplayNode, OptionTypeNode,
    PayerValueNode, PdaLinkNode, PdaNode, PdaSeedNode, PdaSeedValueNode, PdaSeedValueValue,
    PdaValueNode, PdaValuePda, PdaValueProgramId, PluginNode, PostOffsetTransformNode,
    PreOffsetTransformNode, PrefixedCountNode, ProgramIdValueNode, ProgramLinkNode, ProgramNode,
    ProvidedNode, PublicKeyTypeNode, PublicKeyValueNode, RemainderCountNode,
    RemainderOptionTypeNode, RootNode, SentinelCountNode, SentinelTransformNode, SetTypeNode,
    SetValueNode, SizeDiscriminatorNode, SizePrefixTransformNode, SomeValueNode, StringDisplayNode,
    StringTypeNode, StringValueNode, StructFieldDisplayNode, StructFieldTypeNode,
    StructFieldValueNode, StructTypeNode, StructValueNode, Text, TextNode, TransformNode,
    TupleTypeNode, TupleValueNode, TypeNode, UnitNumberDisplayNode, ValueLabelDisplayNode,
    ValueNode, VariablePdaSeedNode, ZeroableOptionTypeNode,
};
use codama_errors::{CodamaError, CodamaResult};
use codama_nodes_derive::NodeUnion;
use derive_more::From;

/// Any node of the Codama IDL, with one variant per node kind.
#[derive(NodeUnion, From, Debug, PartialEq, Clone)]
#[non_exhaustive]
pub enum Node {
    Account(AccountNode),
    AccountBumpValue(AccountBumpValueNode),
    AccountDataValue(AccountDataValueNode),
    AccountLink(AccountLinkNode),
    AccountValue(AccountValueNode),
    AmountNumberDisplay(AmountNumberDisplayNode),
    ArrayType(ArrayTypeNode),
    ArrayValue(ArrayValueNode),
    BooleanType(BooleanTypeNode),
    BooleanValue(BooleanValueNode),
    BytesType(BytesTypeNode),
    BytesValue(BytesValueNode),
    ConditionalValue(ConditionalValueNode),
    Constant(ConstantNode),
    ConstantDiscriminator(ConstantDiscriminatorNode),
    ConstantPdaSeed(ConstantPdaSeedNode),
    ConstantValue(ConstantValueNode),
    DataValue(DataValueNode),
    DateTimeType(DateTimeTypeNode),
    DefinedType(DefinedTypeNode),
    DefinedTypeLink(DefinedTypeLinkNode),
    DurationType(DurationTypeNode),
    EnumType(EnumTypeNode),
    EnumValue(EnumValueNode),
    EnumVariantDisplay(EnumVariantDisplayNode),
    EnumVariantType(EnumVariantTypeNode),
    Error(ErrorNode),
    Event(EventNode),
    FieldDiscriminator(FieldDiscriminatorNode),
    FixedCount(FixedCountNode),
    FixedPointType(FixedPointTypeNode),
    FixedSizeTransform(FixedSizeTransformNode),
    FloatType(FloatTypeNode),
    FloatValue(FloatValueNode),
    HiddenPrefixTransform(HiddenPrefixTransformNode),
    HiddenSuffixTransform(HiddenSuffixTransformNode),
    IdentityValue(IdentityValueNode),
    InjectedValue(InjectedValueNode),
    Instruction(InstructionNode),
    InstructionAccount(InstructionAccountNode),
    InstructionAccountDisplay(InstructionAccountDisplayNode),
    InstructionAccountLink(InstructionAccountLinkNode),
    InstructionByteDelta(InstructionByteDeltaNode),
    InstructionDisplay(InstructionDisplayNode),
    InstructionLink(InstructionLinkNode),
    InstructionRemainingAccounts(InstructionRemainingAccountsNode),
    InstructionStatus(InstructionStatusNode),
    IntegerType(IntegerTypeNode),
    IntegerValue(IntegerValueNode),
    MapEntryValue(MapEntryValueNode),
    MapType(MapTypeNode),
    MapValue(MapValueNode),
    NoneValue(NoneValueNode),
    OptionType(OptionTypeNode),
    PayerValue(PayerValueNode),
    Pda(PdaNode),
    PdaLink(PdaLinkNode),
    PdaSeedValue(PdaSeedValueNode),
    PdaValue(PdaValueNode),
    Plugin(PluginNode),
    PostOffsetTransform(PostOffsetTransformNode),
    PrefixedCount(PrefixedCountNode),
    PreOffsetTransform(PreOffsetTransformNode),
    Program(ProgramNode),
    ProgramIdValue(ProgramIdValueNode),
    ProgramLink(ProgramLinkNode),
    Provided(ProvidedNode),
    PublicKeyType(PublicKeyTypeNode),
    PublicKeyValue(PublicKeyValueNode),
    RemainderCount(RemainderCountNode),
    RemainderOptionType(RemainderOptionTypeNode),
    Root(RootNode),
    SentinelCount(SentinelCountNode),
    SentinelTransform(SentinelTransformNode),
    SetType(SetTypeNode),
    SetValue(SetValueNode),
    SizeDiscriminator(SizeDiscriminatorNode),
    SizePrefixTransform(SizePrefixTransformNode),
    SomeValue(SomeValueNode),
    StringDisplay(StringDisplayNode),
    StringType(StringTypeNode),
    StringValue(StringValueNode),
    StructFieldDisplay(StructFieldDisplayNode),
    StructFieldType(StructFieldTypeNode),
    StructFieldValue(StructFieldValueNode),
    StructType(StructTypeNode),
    StructValue(StructValueNode),
    Text(TextNode),
    TupleType(TupleTypeNode),
    TupleValue(TupleValueNode),
    UnitNumberDisplay(UnitNumberDisplayNode),
    ValueLabelDisplay(ValueLabelDisplayNode),
    VariablePdaSeed(VariablePdaSeedNode),
    ZeroableOptionType(ZeroableOptionTypeNode),
}

/// A borrowed node, with one variant per node kind, to walk a tree without cloning it.
#[derive(Debug, PartialEq, Clone, Copy)]
#[non_exhaustive]
pub enum NodeRef<'a> {
    Account(&'a AccountNode),
    AccountBumpValue(&'a AccountBumpValueNode),
    AccountDataValue(&'a AccountDataValueNode),
    AccountLink(&'a AccountLinkNode),
    AccountValue(&'a AccountValueNode),
    AmountNumberDisplay(&'a AmountNumberDisplayNode),
    ArrayType(&'a ArrayTypeNode),
    ArrayValue(&'a ArrayValueNode),
    BooleanType(&'a BooleanTypeNode),
    BooleanValue(&'a BooleanValueNode),
    BytesType(&'a BytesTypeNode),
    BytesValue(&'a BytesValueNode),
    ConditionalValue(&'a ConditionalValueNode),
    Constant(&'a ConstantNode),
    ConstantDiscriminator(&'a ConstantDiscriminatorNode),
    ConstantPdaSeed(&'a ConstantPdaSeedNode),
    ConstantValue(&'a ConstantValueNode),
    DataValue(&'a DataValueNode),
    DateTimeType(&'a DateTimeTypeNode),
    DefinedType(&'a DefinedTypeNode),
    DefinedTypeLink(&'a DefinedTypeLinkNode),
    DurationType(&'a DurationTypeNode),
    EnumType(&'a EnumTypeNode),
    EnumValue(&'a EnumValueNode),
    EnumVariantDisplay(&'a EnumVariantDisplayNode),
    EnumVariantType(&'a EnumVariantTypeNode),
    Error(&'a ErrorNode),
    Event(&'a EventNode),
    FieldDiscriminator(&'a FieldDiscriminatorNode),
    FixedCount(&'a FixedCountNode),
    FixedPointType(&'a FixedPointTypeNode),
    FixedSizeTransform(&'a FixedSizeTransformNode),
    FloatType(&'a FloatTypeNode),
    FloatValue(&'a FloatValueNode),
    HiddenPrefixTransform(&'a HiddenPrefixTransformNode),
    HiddenSuffixTransform(&'a HiddenSuffixTransformNode),
    IdentityValue(&'a IdentityValueNode),
    InjectedValue(&'a InjectedValueNode),
    Instruction(&'a InstructionNode),
    InstructionAccount(&'a InstructionAccountNode),
    InstructionAccountDisplay(&'a InstructionAccountDisplayNode),
    InstructionAccountLink(&'a InstructionAccountLinkNode),
    InstructionByteDelta(&'a InstructionByteDeltaNode),
    InstructionDisplay(&'a InstructionDisplayNode),
    InstructionLink(&'a InstructionLinkNode),
    InstructionRemainingAccounts(&'a InstructionRemainingAccountsNode),
    InstructionStatus(&'a InstructionStatusNode),
    IntegerType(&'a IntegerTypeNode),
    IntegerValue(&'a IntegerValueNode),
    MapEntryValue(&'a MapEntryValueNode),
    MapType(&'a MapTypeNode),
    MapValue(&'a MapValueNode),
    NoneValue(&'a NoneValueNode),
    OptionType(&'a OptionTypeNode),
    PayerValue(&'a PayerValueNode),
    Pda(&'a PdaNode),
    PdaLink(&'a PdaLinkNode),
    PdaSeedValue(&'a PdaSeedValueNode),
    PdaValue(&'a PdaValueNode),
    Plugin(&'a PluginNode),
    PostOffsetTransform(&'a PostOffsetTransformNode),
    PrefixedCount(&'a PrefixedCountNode),
    PreOffsetTransform(&'a PreOffsetTransformNode),
    Program(&'a ProgramNode),
    ProgramIdValue(&'a ProgramIdValueNode),
    ProgramLink(&'a ProgramLinkNode),
    Provided(&'a ProvidedNode),
    PublicKeyType(&'a PublicKeyTypeNode),
    PublicKeyValue(&'a PublicKeyValueNode),
    RemainderCount(&'a RemainderCountNode),
    RemainderOptionType(&'a RemainderOptionTypeNode),
    Root(&'a RootNode),
    SentinelCount(&'a SentinelCountNode),
    SentinelTransform(&'a SentinelTransformNode),
    SetType(&'a SetTypeNode),
    SetValue(&'a SetValueNode),
    SizeDiscriminator(&'a SizeDiscriminatorNode),
    SizePrefixTransform(&'a SizePrefixTransformNode),
    SomeValue(&'a SomeValueNode),
    StringDisplay(&'a StringDisplayNode),
    StringType(&'a StringTypeNode),
    StringValue(&'a StringValueNode),
    StructFieldDisplay(&'a StructFieldDisplayNode),
    StructFieldType(&'a StructFieldTypeNode),
    StructFieldValue(&'a StructFieldValueNode),
    StructType(&'a StructTypeNode),
    StructValue(&'a StructValueNode),
    Text(&'a TextNode),
    TupleType(&'a TupleTypeNode),
    TupleValue(&'a TupleValueNode),
    UnitNumberDisplay(&'a UnitNumberDisplayNode),
    ValueLabelDisplay(&'a ValueLabelDisplayNode),
    VariablePdaSeed(&'a VariablePdaSeedNode),
    ZeroableOptionType(&'a ZeroableOptionTypeNode),
}

impl<'a> From<&'a Node> for NodeRef<'a> {
    fn from(node: &'a Node) -> Self {
        match node {
            Node::Account(node) => Self::Account(node),
            Node::AccountBumpValue(node) => Self::AccountBumpValue(node),
            Node::AccountDataValue(node) => Self::AccountDataValue(node),
            Node::AccountLink(node) => Self::AccountLink(node),
            Node::AccountValue(node) => Self::AccountValue(node),
            Node::AmountNumberDisplay(node) => Self::AmountNumberDisplay(node),
            Node::ArrayType(node) => Self::ArrayType(node),
            Node::ArrayValue(node) => Self::ArrayValue(node),
            Node::BooleanType(node) => Self::BooleanType(node),
            Node::BooleanValue(node) => Self::BooleanValue(node),
            Node::BytesType(node) => Self::BytesType(node),
            Node::BytesValue(node) => Self::BytesValue(node),
            Node::ConditionalValue(node) => Self::ConditionalValue(node),
            Node::Constant(node) => Self::Constant(node),
            Node::ConstantDiscriminator(node) => Self::ConstantDiscriminator(node),
            Node::ConstantPdaSeed(node) => Self::ConstantPdaSeed(node),
            Node::ConstantValue(node) => Self::ConstantValue(node),
            Node::DataValue(node) => Self::DataValue(node),
            Node::DateTimeType(node) => Self::DateTimeType(node),
            Node::DefinedType(node) => Self::DefinedType(node),
            Node::DefinedTypeLink(node) => Self::DefinedTypeLink(node),
            Node::DurationType(node) => Self::DurationType(node),
            Node::EnumType(node) => Self::EnumType(node),
            Node::EnumValue(node) => Self::EnumValue(node),
            Node::EnumVariantDisplay(node) => Self::EnumVariantDisplay(node),
            Node::EnumVariantType(node) => Self::EnumVariantType(node),
            Node::Error(node) => Self::Error(node),
            Node::Event(node) => Self::Event(node),
            Node::FieldDiscriminator(node) => Self::FieldDiscriminator(node),
            Node::FixedCount(node) => Self::FixedCount(node),
            Node::FixedPointType(node) => Self::FixedPointType(node),
            Node::FixedSizeTransform(node) => Self::FixedSizeTransform(node),
            Node::FloatType(node) => Self::FloatType(node),
            Node::FloatValue(node) => Self::FloatValue(node),
            Node::HiddenPrefixTransform(node) => Self::HiddenPrefixTransform(node),
            Node::HiddenSuffixTransform(node) => Self::HiddenSuffixTransform(node),
            Node::IdentityValue(node) => Self::IdentityValue(node),
            Node::InjectedValue(node) => Self::InjectedValue(node),
            Node::Instruction(node) => Self::Instruction(node),
            Node::InstructionAccount(node) => Self::InstructionAccount(node),
            Node::InstructionAccountDisplay(node) => Self::InstructionAccountDisplay(node),
            Node::InstructionAccountLink(node) => Self::InstructionAccountLink(node),
            Node::InstructionByteDelta(node) => Self::InstructionByteDelta(node),
            Node::InstructionDisplay(node) => Self::InstructionDisplay(node),
            Node::InstructionLink(node) => Self::InstructionLink(node),
            Node::InstructionRemainingAccounts(node) => Self::InstructionRemainingAccounts(node),
            Node::InstructionStatus(node) => Self::InstructionStatus(node),
            Node::IntegerType(node) => Self::IntegerType(node),
            Node::IntegerValue(node) => Self::IntegerValue(node),
            Node::MapEntryValue(node) => Self::MapEntryValue(node),
            Node::MapType(node) => Self::MapType(node),
            Node::MapValue(node) => Self::MapValue(node),
            Node::NoneValue(node) => Self::NoneValue(node),
            Node::OptionType(node) => Self::OptionType(node),
            Node::PayerValue(node) => Self::PayerValue(node),
            Node::Pda(node) => Self::Pda(node),
            Node::PdaLink(node) => Self::PdaLink(node),
            Node::PdaSeedValue(node) => Self::PdaSeedValue(node),
            Node::PdaValue(node) => Self::PdaValue(node),
            Node::Plugin(node) => Self::Plugin(node),
            Node::PostOffsetTransform(node) => Self::PostOffsetTransform(node),
            Node::PrefixedCount(node) => Self::PrefixedCount(node),
            Node::PreOffsetTransform(node) => Self::PreOffsetTransform(node),
            Node::Program(node) => Self::Program(node),
            Node::ProgramIdValue(node) => Self::ProgramIdValue(node),
            Node::ProgramLink(node) => Self::ProgramLink(node),
            Node::Provided(node) => Self::Provided(node),
            Node::PublicKeyType(node) => Self::PublicKeyType(node),
            Node::PublicKeyValue(node) => Self::PublicKeyValue(node),
            Node::RemainderCount(node) => Self::RemainderCount(node),
            Node::RemainderOptionType(node) => Self::RemainderOptionType(node),
            Node::Root(node) => Self::Root(node),
            Node::SentinelCount(node) => Self::SentinelCount(node),
            Node::SentinelTransform(node) => Self::SentinelTransform(node),
            Node::SetType(node) => Self::SetType(node),
            Node::SetValue(node) => Self::SetValue(node),
            Node::SizeDiscriminator(node) => Self::SizeDiscriminator(node),
            Node::SizePrefixTransform(node) => Self::SizePrefixTransform(node),
            Node::SomeValue(node) => Self::SomeValue(node),
            Node::StringDisplay(node) => Self::StringDisplay(node),
            Node::StringType(node) => Self::StringType(node),
            Node::StringValue(node) => Self::StringValue(node),
            Node::StructFieldDisplay(node) => Self::StructFieldDisplay(node),
            Node::StructFieldType(node) => Self::StructFieldType(node),
            Node::StructFieldValue(node) => Self::StructFieldValue(node),
            Node::StructType(node) => Self::StructType(node),
            Node::StructValue(node) => Self::StructValue(node),
            Node::Text(node) => Self::Text(node),
            Node::TupleType(node) => Self::TupleType(node),
            Node::TupleValue(node) => Self::TupleValue(node),
            Node::UnitNumberDisplay(node) => Self::UnitNumberDisplay(node),
            Node::ValueLabelDisplay(node) => Self::ValueLabelDisplay(node),
            Node::VariablePdaSeed(node) => Self::VariablePdaSeed(node),
            Node::ZeroableOptionType(node) => Self::ZeroableOptionType(node),
        }
    }
}

impl<'a> NodeRef<'a> {
    /// Clones the borrowed node into an owned `Node`.
    pub fn to_node(self) -> Node {
        match self {
            Self::Account(node) => Node::Account(node.clone()),
            Self::AccountBumpValue(node) => Node::AccountBumpValue(node.clone()),
            Self::AccountDataValue(node) => Node::AccountDataValue(node.clone()),
            Self::AccountLink(node) => Node::AccountLink(node.clone()),
            Self::AccountValue(node) => Node::AccountValue(node.clone()),
            Self::AmountNumberDisplay(node) => Node::AmountNumberDisplay(node.clone()),
            Self::ArrayType(node) => Node::ArrayType(node.clone()),
            Self::ArrayValue(node) => Node::ArrayValue(node.clone()),
            Self::BooleanType(node) => Node::BooleanType(node.clone()),
            Self::BooleanValue(node) => Node::BooleanValue(node.clone()),
            Self::BytesType(node) => Node::BytesType(node.clone()),
            Self::BytesValue(node) => Node::BytesValue(node.clone()),
            Self::ConditionalValue(node) => Node::ConditionalValue(node.clone()),
            Self::Constant(node) => Node::Constant(node.clone()),
            Self::ConstantDiscriminator(node) => Node::ConstantDiscriminator(node.clone()),
            Self::ConstantPdaSeed(node) => Node::ConstantPdaSeed(node.clone()),
            Self::ConstantValue(node) => Node::ConstantValue(node.clone()),
            Self::DataValue(node) => Node::DataValue(node.clone()),
            Self::DateTimeType(node) => Node::DateTimeType(node.clone()),
            Self::DefinedType(node) => Node::DefinedType(node.clone()),
            Self::DefinedTypeLink(node) => Node::DefinedTypeLink(node.clone()),
            Self::DurationType(node) => Node::DurationType(node.clone()),
            Self::EnumType(node) => Node::EnumType(node.clone()),
            Self::EnumValue(node) => Node::EnumValue(node.clone()),
            Self::EnumVariantDisplay(node) => Node::EnumVariantDisplay(node.clone()),
            Self::EnumVariantType(node) => Node::EnumVariantType(node.clone()),
            Self::Error(node) => Node::Error(node.clone()),
            Self::Event(node) => Node::Event(node.clone()),
            Self::FieldDiscriminator(node) => Node::FieldDiscriminator(node.clone()),
            Self::FixedCount(node) => Node::FixedCount(node.clone()),
            Self::FixedPointType(node) => Node::FixedPointType(node.clone()),
            Self::FixedSizeTransform(node) => Node::FixedSizeTransform(node.clone()),
            Self::FloatType(node) => Node::FloatType(node.clone()),
            Self::FloatValue(node) => Node::FloatValue(node.clone()),
            Self::HiddenPrefixTransform(node) => Node::HiddenPrefixTransform(node.clone()),
            Self::HiddenSuffixTransform(node) => Node::HiddenSuffixTransform(node.clone()),
            Self::IdentityValue(node) => Node::IdentityValue(node.clone()),
            Self::InjectedValue(node) => Node::InjectedValue(node.clone()),
            Self::Instruction(node) => Node::Instruction(node.clone()),
            Self::InstructionAccount(node) => Node::InstructionAccount(node.clone()),
            Self::InstructionAccountDisplay(node) => Node::InstructionAccountDisplay(node.clone()),
            Self::InstructionAccountLink(node) => Node::InstructionAccountLink(node.clone()),
            Self::InstructionByteDelta(node) => Node::InstructionByteDelta(node.clone()),
            Self::InstructionDisplay(node) => Node::InstructionDisplay(node.clone()),
            Self::InstructionLink(node) => Node::InstructionLink(node.clone()),
            Self::InstructionRemainingAccounts(node) => {
                Node::InstructionRemainingAccounts(node.clone())
            }
            Self::InstructionStatus(node) => Node::InstructionStatus(node.clone()),
            Self::IntegerType(node) => Node::IntegerType(node.clone()),
            Self::IntegerValue(node) => Node::IntegerValue(node.clone()),
            Self::MapEntryValue(node) => Node::MapEntryValue(node.clone()),
            Self::MapType(node) => Node::MapType(node.clone()),
            Self::MapValue(node) => Node::MapValue(node.clone()),
            Self::NoneValue(node) => Node::NoneValue(node.clone()),
            Self::OptionType(node) => Node::OptionType(node.clone()),
            Self::PayerValue(node) => Node::PayerValue(node.clone()),
            Self::Pda(node) => Node::Pda(node.clone()),
            Self::PdaLink(node) => Node::PdaLink(node.clone()),
            Self::PdaSeedValue(node) => Node::PdaSeedValue(node.clone()),
            Self::PdaValue(node) => Node::PdaValue(node.clone()),
            Self::Plugin(node) => Node::Plugin(node.clone()),
            Self::PostOffsetTransform(node) => Node::PostOffsetTransform(node.clone()),
            Self::PrefixedCount(node) => Node::PrefixedCount(node.clone()),
            Self::PreOffsetTransform(node) => Node::PreOffsetTransform(node.clone()),
            Self::Program(node) => Node::Program(node.clone()),
            Self::ProgramIdValue(node) => Node::ProgramIdValue(node.clone()),
            Self::ProgramLink(node) => Node::ProgramLink(node.clone()),
            Self::Provided(node) => Node::Provided(node.clone()),
            Self::PublicKeyType(node) => Node::PublicKeyType(node.clone()),
            Self::PublicKeyValue(node) => Node::PublicKeyValue(node.clone()),
            Self::RemainderCount(node) => Node::RemainderCount(node.clone()),
            Self::RemainderOptionType(node) => Node::RemainderOptionType(node.clone()),
            Self::Root(node) => Node::Root(node.clone()),
            Self::SentinelCount(node) => Node::SentinelCount(node.clone()),
            Self::SentinelTransform(node) => Node::SentinelTransform(node.clone()),
            Self::SetType(node) => Node::SetType(node.clone()),
            Self::SetValue(node) => Node::SetValue(node.clone()),
            Self::SizeDiscriminator(node) => Node::SizeDiscriminator(node.clone()),
            Self::SizePrefixTransform(node) => Node::SizePrefixTransform(node.clone()),
            Self::SomeValue(node) => Node::SomeValue(node.clone()),
            Self::StringDisplay(node) => Node::StringDisplay(node.clone()),
            Self::StringType(node) => Node::StringType(node.clone()),
            Self::StringValue(node) => Node::StringValue(node.clone()),
            Self::StructFieldDisplay(node) => Node::StructFieldDisplay(node.clone()),
            Self::StructFieldType(node) => Node::StructFieldType(node.clone()),
            Self::StructFieldValue(node) => Node::StructFieldValue(node.clone()),
            Self::StructType(node) => Node::StructType(node.clone()),
            Self::StructValue(node) => Node::StructValue(node.clone()),
            Self::Text(node) => Node::Text(node.clone()),
            Self::TupleType(node) => Node::TupleType(node.clone()),
            Self::TupleValue(node) => Node::TupleValue(node.clone()),
            Self::UnitNumberDisplay(node) => Node::UnitNumberDisplay(node.clone()),
            Self::ValueLabelDisplay(node) => Node::ValueLabelDisplay(node.clone()),
            Self::VariablePdaSeed(node) => Node::VariablePdaSeed(node.clone()),
            Self::ZeroableOptionType(node) => Node::ZeroableOptionType(node.clone()),
        }
    }

    /// The identifier of the node, if it has one.
    pub fn identifier(self) -> Option<&'a Identifier> {
        match self {
            Self::Account(node) => Some(&node.identifier),
            Self::AccountBumpValue(node) => Some(&node.identifier),
            Self::AccountLink(node) => Some(&node.identifier),
            Self::AccountValue(node) => Some(&node.identifier),
            Self::Constant(node) => Some(&node.identifier),
            Self::DefinedType(node) => Some(&node.identifier),
            Self::DefinedTypeLink(node) => Some(&node.identifier),
            Self::EnumVariantType(node) => Some(&node.identifier),
            Self::Error(node) => Some(&node.identifier),
            Self::Event(node) => Some(&node.identifier),
            Self::Instruction(node) => Some(&node.identifier),
            Self::InstructionAccount(node) => Some(&node.identifier),
            Self::InstructionAccountLink(node) => Some(&node.identifier),
            Self::InstructionLink(node) => Some(&node.identifier),
            Self::InstructionRemainingAccounts(node) => Some(&node.identifier),
            Self::Pda(node) => Some(&node.identifier),
            Self::PdaLink(node) => Some(&node.identifier),
            Self::PdaSeedValue(node) => Some(&node.identifier),
            Self::Program(node) => Some(&node.identifier),
            Self::ProgramLink(node) => Some(&node.identifier),
            Self::Provided(node) => Some(&node.identifier),
            Self::PublicKeyValue(node) => node.identifier.as_ref(),
            Self::StructFieldType(node) => Some(&node.identifier),
            Self::StructFieldValue(node) => Some(&node.identifier),
            Self::VariablePdaSeed(node) => Some(&node.identifier),
            _ => None,
        }
    }

    /// The child nodes of the node, in the order of the spec attributes that hold them.
    pub fn children(self) -> Vec<NodeRef<'a>> {
        let mut children = Vec::new();
        match self {
            Self::Account(node) => {
                if let Some(Text::Node(child)) = &*node.docs {
                    children.push(NodeRef::from(child));
                }
                children.push(NodeRef::from(&*node.data));
                children.extend(node.pda.as_ref().map(NodeRef::from));
                children.extend(node.discriminators.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::AccountBumpValue(node) => {
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::AccountDataValue(node) => {
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::AccountLink(node) => {
                children.extend(node.program.as_ref().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::AccountValue(node) => {
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::AmountNumberDisplay(node) => {
                children.push(NodeRef::from(&*node.decimals));
                children.extend((*node.unit).as_ref().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::ArrayType(node) => {
                children.push(NodeRef::from(&*node.item));
                children.push(NodeRef::from(&*node.count));
                children.extend(node.transforms.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::ArrayValue(node) => {
                children.extend(node.items.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::BooleanType(node) => {
                children.push(NodeRef::from(&node.size));
                children.extend(node.transforms.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::BooleanValue(node) => {
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::BytesType(node) => {
                children.extend(node.transforms.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::BytesValue(node) => {
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::ConditionalValue(node) => {
                children.push(NodeRef::from(&*node.condition));
                children.extend((*node.value).as_ref().map(NodeRef::from));
                children.extend((*node.if_true).as_ref().map(NodeRef::from));
                children.extend((*node.if_false).as_ref().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::Constant(node) => {
                if let Some(Text::Node(child)) = &*node.docs {
                    children.push(NodeRef::from(child));
                }
                children.push(NodeRef::from(&*node.r#type));
                children.push(NodeRef::from(&*node.value));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::ConstantDiscriminator(node) => {
                children.push(NodeRef::from(&node.constant));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::ConstantPdaSeed(node) => {
                children.push(NodeRef::from(&*node.r#type));
                children.push(NodeRef::from(&*node.value));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::ConstantValue(node) => {
                children.push(NodeRef::from(&*node.r#type));
                children.push(NodeRef::from(&*node.value));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::DataValue(node) => {
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::DateTimeType(node) => {
                children.push(NodeRef::from(&node.number));
                children.extend(node.transforms.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::DefinedType(node) => {
                if let Some(Text::Node(child)) = &*node.docs {
                    children.push(NodeRef::from(child));
                }
                children.push(NodeRef::from(&*node.r#type));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::DefinedTypeLink(node) => {
                children.extend(node.program.as_ref().map(NodeRef::from));
                children.extend(node.transforms.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::DurationType(node) => {
                children.push(NodeRef::from(&node.number));
                children.extend(node.transforms.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::EnumType(node) => {
                children.extend(node.variants.iter().map(NodeRef::from));
                children.push(NodeRef::from(&node.size));
                children.extend(node.transforms.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::EnumValue(node) => {
                children.push(NodeRef::from(&node.r#enum));
                children.extend((*node.value).as_ref().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::EnumVariantDisplay(node) => {
                if let Some(Text::Node(child)) = &node.label {
                    children.push(NodeRef::from(child));
                }
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::EnumVariantType(node) => {
                if let Some(Text::Node(child)) = &*node.docs {
                    children.push(NodeRef::from(child));
                }
                children.extend((*node.data).as_ref().map(NodeRef::from));
                children.extend(node.display.as_ref().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::Error(node) => {
                if let Text::Node(child) = &node.message {
                    children.push(NodeRef::from(child));
                }
                if let Some(Text::Node(child)) = &*node.docs {
                    children.push(NodeRef::from(child));
                }
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::Event(node) => {
                if let Some(Text::Node(child)) = &*node.docs {
                    children.push(NodeRef::from(child));
                }
                children.push(NodeRef::from(&*node.data));
                children.extend(node.discriminators.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::FieldDiscriminator(node) => {
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::FixedCount(node) => {
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::FixedPointType(node) => {
                children.push(NodeRef::from(&node.number));
                children.extend(node.display.as_ref().map(NodeRef::from));
                children.extend(node.transforms.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::FixedSizeTransform(node) => {
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::FloatType(node) => {
                children.extend(node.display.as_ref().map(NodeRef::from));
                children.extend(node.transforms.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::FloatValue(node) => {
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::HiddenPrefixTransform(node) => {
                children.extend(node.prefix.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::HiddenSuffixTransform(node) => {
                children.extend(node.suffix.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::IdentityValue(node) => {
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::InjectedValue(node) => {
                children.extend((*node.fallback).as_ref().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::Instruction(node) => {
                if let Some(Text::Node(child)) = &*node.docs {
                    children.push(NodeRef::from(child));
                }
                children.extend(node.accounts.iter().map(NodeRef::from));
                children.extend((*node.data).as_ref().map(NodeRef::from));
                children.extend((*node.return_data).as_ref().map(NodeRef::from));
                children.extend(node.remaining_accounts.iter().map(NodeRef::from));
                children.extend(node.byte_deltas.iter().map(NodeRef::from));
                children.extend(node.discriminators.iter().map(NodeRef::from));
                children.extend(node.status.as_ref().map(NodeRef::from));
                children.extend(node.sub_instructions.iter().map(NodeRef::from));
                children.extend(node.provides.iter().map(NodeRef::from));
                children.extend(node.display.as_ref().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::InstructionAccount(node) => {
                if let Some(Text::Node(child)) = &*node.docs {
                    children.push(NodeRef::from(child));
                }
                children.extend((*node.default_value).as_ref().map(NodeRef::from));
                children.extend(node.account_link.as_ref().map(NodeRef::from));
                children.extend(node.display.as_ref().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::InstructionAccountDisplay(node) => {
                if let Some(Text::Node(child)) = &node.label {
                    children.push(NodeRef::from(child));
                }
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::InstructionAccountLink(node) => {
                children.extend(node.instruction.as_ref().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::InstructionByteDelta(node) => {
                children.push(NodeRef::from(&*node.value));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::InstructionDisplay(node) => {
                if let Some(Text::Node(child)) = &node.intent {
                    children.push(NodeRef::from(child));
                }
                if let Some(Text::Node(child)) = &node.interpolated_intent {
                    children.push(NodeRef::from(child));
                }
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::InstructionLink(node) => {
                children.extend(node.program.as_ref().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::InstructionRemainingAccounts(node) => {
                if let Some(Text::Node(child)) = &*node.docs {
                    children.push(NodeRef::from(child));
                }
                children.extend(node.display.as_ref().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::InstructionStatus(node) => {
                if let Some(Text::Node(child)) = &node.message {
                    children.push(NodeRef::from(child));
                }
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::IntegerType(node) => {
                children.extend((*node.display).as_ref().map(NodeRef::from));
                children.extend(node.transforms.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::IntegerValue(node) => {
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::MapEntryValue(node) => {
                children.push(NodeRef::from(&*node.key));
                children.push(NodeRef::from(&*node.value));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::MapType(node) => {
                children.push(NodeRef::from(&*node.key));
                children.push(NodeRef::from(&*node.value));
                children.push(NodeRef::from(&*node.count));
                children.extend(node.transforms.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::MapValue(node) => {
                children.extend(node.entries.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::NoneValue(node) => {
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::OptionType(node) => {
                children.push(NodeRef::from(&*node.item));
                children.push(NodeRef::from(&node.prefix));
                children.extend(node.transforms.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::PayerValue(node) => {
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::Pda(node) => {
                if let Some(Text::Node(child)) = &*node.docs {
                    children.push(NodeRef::from(child));
                }
                children.extend(node.seeds.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::PdaLink(node) => {
                children.extend(node.program.as_ref().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::PdaSeedValue(node) => {
                children.push(NodeRef::from(&*node.value));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::PdaValue(node) => {
                children.push(NodeRef::from(&*node.pda));
                children.extend(node.seeds.iter().map(NodeRef::from));
                children.extend((*node.program_id).as_ref().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::Plugin(node) => {
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::PostOffsetTransform(node) => {
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::PrefixedCount(node) => {
                children.push(NodeRef::from(&node.prefix));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::PreOffsetTransform(node) => {
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::Program(node) => {
                if let Some(Text::Node(child)) = &*node.docs {
                    children.push(NodeRef::from(child));
                }
                children.extend(node.accounts.iter().map(NodeRef::from));
                children.extend(node.instructions.iter().map(NodeRef::from));
                children.extend(node.defined_types.iter().map(NodeRef::from));
                children.extend(node.pdas.iter().map(NodeRef::from));
                children.extend(node.events.iter().map(NodeRef::from));
                children.extend(node.errors.iter().map(NodeRef::from));
                children.extend(node.constants.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::ProgramIdValue(node) => {
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::ProgramLink(node) => {
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::Provided(node) => {
                children.push(NodeRef::from(&*node.node));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::PublicKeyType(node) => {
                children.extend(node.account_link.as_ref().map(NodeRef::from));
                children.extend(node.transforms.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::PublicKeyValue(node) => {
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::RemainderCount(node) => {
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::RemainderOptionType(node) => {
                children.push(NodeRef::from(&*node.item));
                children.extend(node.transforms.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::Root(node) => {
                children.push(NodeRef::from(&node.program));
                children.extend(node.additional_programs.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::SentinelCount(node) => {
                children.push(NodeRef::from(&node.sentinel));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::SentinelTransform(node) => {
                children.push(NodeRef::from(&node.sentinel));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::SetType(node) => {
                children.push(NodeRef::from(&*node.item));
                children.push(NodeRef::from(&*node.count));
                children.extend(node.transforms.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::SetValue(node) => {
                children.extend(node.items.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::SizeDiscriminator(node) => {
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::SizePrefixTransform(node) => {
                children.push(NodeRef::from(&node.prefix));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::SomeValue(node) => {
                children.push(NodeRef::from(&*node.value));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::StringDisplay(node) => {
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::StringType(node) => {
                children.extend(node.display.as_ref().map(NodeRef::from));
                children.extend(node.transforms.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::StringValue(node) => {
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::StructFieldDisplay(node) => {
                if let Some(Text::Node(child)) = &node.label {
                    children.push(NodeRef::from(child));
                }
                if let Some(Text::Node(child)) = &node.flatten_prefix {
                    children.push(NodeRef::from(child));
                }
                children.extend(node.value_labels.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::StructFieldType(node) => {
                if let Some(Text::Node(child)) = &*node.docs {
                    children.push(NodeRef::from(child));
                }
                children.push(NodeRef::from(&*node.r#type));
                children.extend((*node.default_value).as_ref().map(NodeRef::from));
                children.extend(node.display.as_ref().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::StructFieldValue(node) => {
                children.push(NodeRef::from(&*node.value));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::StructType(node) => {
                children.extend(node.fields.iter().map(NodeRef::from));
                children.extend(node.transforms.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::StructValue(node) => {
                children.extend(node.fields.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::Text(node) => {
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::TupleType(node) => {
                children.extend(node.items.iter().map(NodeRef::from));
                children.extend(node.transforms.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::TupleValue(node) => {
                children.extend(node.items.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::UnitNumberDisplay(node) => {
                children.push(NodeRef::from(&*node.unit));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::ValueLabelDisplay(node) => {
                if let Text::Node(child) = &node.label {
                    children.push(NodeRef::from(child));
                }
                children.push(NodeRef::from(&*node.value));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::VariablePdaSeed(node) => {
                if let Some(Text::Node(child)) = &*node.docs {
                    children.push(NodeRef::from(child));
                }
                children.push(NodeRef::from(&*node.r#type));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
            Self::ZeroableOptionType(node) => {
                children.push(NodeRef::from(&*node.item));
                children.extend(node.zero_value.as_ref().map(NodeRef::from));
                children.extend(node.transforms.iter().map(NodeRef::from));
                children.extend(node.plugins.iter().map(NodeRef::from));
            }
        }
        children
    }

    /// Rebuilds the node from its children mapped by `f`, in the order of the spec attributes that hold them.
    ///
    /// When `f` returns `None`, the child is deleted: an item is dropped from its list, an optional
    /// child becomes `None`, and a required child deletes this node too, which returns `None`.
    /// A mapped child of the wrong kind for its slot is an error.
    // Data fields are cloned whatever their type, as the generator does not track which ones are `Copy`.
    #[allow(clippy::clone_on_copy)]
    pub fn map_children(
        self,
        mut f: impl FnMut(NodeRef<'a>) -> CodamaResult<Option<Node>>,
    ) -> CodamaResult<Option<Node>> {
        let f: &mut MapChild<'_, 'a> = &mut f;
        match self {
            Self::Account(this) => {
                let docs = map_docs(&this.docs, f)?;
                let Some(data) = map_required::<TypeNode>(NodeRef::from(&*this.data), f)? else {
                    return Ok(None);
                };
                let pda = map_optional::<PdaLinkNode>(this.pda.as_ref().map(NodeRef::from), f)?;
                let discriminators = map_array::<DiscriminatorNode, _>(&this.discriminators, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::Account(AccountNode {
                    identifier: this.identifier.clone(),
                    size: this.size.clone(),
                    docs,
                    data: Box::new(data),
                    pda,
                    discriminators,
                    plugins,
                })))
            }
            Self::AccountBumpValue(this) => {
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::AccountBumpValue(AccountBumpValueNode {
                    identifier: this.identifier.clone(),
                    plugins,
                })))
            }
            Self::AccountDataValue(this) => {
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::AccountDataValue(AccountDataValueNode {
                    account: this.account.clone(),
                    path: this.path.clone(),
                    plugins,
                })))
            }
            Self::AccountLink(this) => {
                let program =
                    map_optional::<ProgramLinkNode>(this.program.as_ref().map(NodeRef::from), f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::AccountLink(AccountLinkNode {
                    identifier: this.identifier.clone(),
                    program,
                    plugins,
                })))
            }
            Self::AccountValue(this) => {
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::AccountValue(AccountValueNode {
                    identifier: this.identifier.clone(),
                    plugins,
                })))
            }
            Self::AmountNumberDisplay(this) => {
                let Some(decimals) =
                    map_required::<InjectableIntegerValueNode>(NodeRef::from(&*this.decimals), f)?
                else {
                    return Ok(None);
                };
                let unit = map_optional::<InjectableUnitValueNode>(
                    (*this.unit).as_ref().map(NodeRef::from),
                    f,
                )?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::AmountNumberDisplay(AmountNumberDisplayNode {
                    decimals: Box::new(decimals),
                    unit: Box::new(unit),
                    plugins,
                })))
            }
            Self::ArrayType(this) => {
                let Some(item) = map_required::<TypeNode>(NodeRef::from(&*this.item), f)? else {
                    return Ok(None);
                };
                let Some(count) = map_required::<CountNode>(NodeRef::from(&*this.count), f)? else {
                    return Ok(None);
                };
                let transforms = map_array::<TransformNode, _>(&this.transforms, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::ArrayType(ArrayTypeNode {
                    item: Box::new(item),
                    count: Box::new(count),
                    transforms,
                    plugins,
                })))
            }
            Self::ArrayValue(this) => {
                let items = map_array::<ValueNode, _>(&this.items, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::ArrayValue(ArrayValueNode { items, plugins })))
            }
            Self::BooleanType(this) => {
                let Some(size) = map_required::<IntegerTypeNode>(NodeRef::from(&this.size), f)?
                else {
                    return Ok(None);
                };
                let transforms = map_array::<TransformNode, _>(&this.transforms, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::BooleanType(BooleanTypeNode {
                    size,
                    transforms,
                    plugins,
                })))
            }
            Self::BooleanValue(this) => {
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::BooleanValue(BooleanValueNode {
                    boolean: this.boolean.clone(),
                    plugins,
                })))
            }
            Self::BytesType(this) => {
                let transforms = map_array::<TransformNode, _>(&this.transforms, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::BytesType(BytesTypeNode {
                    transforms,
                    plugins,
                })))
            }
            Self::BytesValue(this) => {
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::BytesValue(BytesValueNode {
                    data: this.data.clone(),
                    encoding: this.encoding.clone(),
                    plugins,
                })))
            }
            Self::ConditionalValue(this) => {
                let Some(condition) =
                    map_required::<ConditionalValueCondition>(NodeRef::from(&*this.condition), f)?
                else {
                    return Ok(None);
                };
                let value =
                    map_optional::<ValueNode>((*this.value).as_ref().map(NodeRef::from), f)?;
                let if_true = map_optional::<InstructionInputValueNode>(
                    (*this.if_true).as_ref().map(NodeRef::from),
                    f,
                )?;
                let if_false = map_optional::<InstructionInputValueNode>(
                    (*this.if_false).as_ref().map(NodeRef::from),
                    f,
                )?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::ConditionalValue(ConditionalValueNode {
                    condition: Box::new(condition),
                    value: Box::new(value),
                    if_true: Box::new(if_true),
                    if_false: Box::new(if_false),
                    plugins,
                })))
            }
            Self::Constant(this) => {
                let docs = map_docs(&this.docs, f)?;
                let Some(r#type) = map_required::<TypeNode>(NodeRef::from(&*this.r#type), f)?
                else {
                    return Ok(None);
                };
                let Some(value) = map_required::<ValueNode>(NodeRef::from(&*this.value), f)? else {
                    return Ok(None);
                };
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::Constant(ConstantNode {
                    identifier: this.identifier.clone(),
                    docs,
                    r#type: Box::new(r#type),
                    value: Box::new(value),
                    plugins,
                })))
            }
            Self::ConstantDiscriminator(this) => {
                let Some(constant) =
                    map_required::<ConstantValueNode>(NodeRef::from(&this.constant), f)?
                else {
                    return Ok(None);
                };
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::ConstantDiscriminator(
                    ConstantDiscriminatorNode {
                        offset: this.offset.clone(),
                        constant,
                        plugins,
                    },
                )))
            }
            Self::ConstantPdaSeed(this) => {
                let Some(r#type) = map_required::<TypeNode>(NodeRef::from(&*this.r#type), f)?
                else {
                    return Ok(None);
                };
                let Some(value) =
                    map_required::<ConstantPdaSeedValue>(NodeRef::from(&*this.value), f)?
                else {
                    return Ok(None);
                };
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::ConstantPdaSeed(ConstantPdaSeedNode {
                    r#type: Box::new(r#type),
                    value: Box::new(value),
                    plugins,
                })))
            }
            Self::ConstantValue(this) => {
                let Some(r#type) = map_required::<TypeNode>(NodeRef::from(&*this.r#type), f)?
                else {
                    return Ok(None);
                };
                let Some(value) = map_required::<ValueNode>(NodeRef::from(&*this.value), f)? else {
                    return Ok(None);
                };
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::ConstantValue(ConstantValueNode {
                    r#type: Box::new(r#type),
                    value: Box::new(value),
                    plugins,
                })))
            }
            Self::DataValue(this) => {
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::DataValue(DataValueNode {
                    path: this.path.clone(),
                    plugins,
                })))
            }
            Self::DateTimeType(this) => {
                let Some(number) = map_required::<IntegerTypeNode>(NodeRef::from(&this.number), f)?
                else {
                    return Ok(None);
                };
                let transforms = map_array::<TransformNode, _>(&this.transforms, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::DateTimeType(DateTimeTypeNode {
                    ticks_per_second: this.ticks_per_second.clone(),
                    number,
                    transforms,
                    plugins,
                })))
            }
            Self::DefinedType(this) => {
                let docs = map_docs(&this.docs, f)?;
                let Some(r#type) = map_required::<TypeNode>(NodeRef::from(&*this.r#type), f)?
                else {
                    return Ok(None);
                };
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::DefinedType(DefinedTypeNode {
                    identifier: this.identifier.clone(),
                    docs,
                    r#type: Box::new(r#type),
                    plugins,
                })))
            }
            Self::DefinedTypeLink(this) => {
                let program =
                    map_optional::<ProgramLinkNode>(this.program.as_ref().map(NodeRef::from), f)?;
                let transforms = map_array::<TransformNode, _>(&this.transforms, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::DefinedTypeLink(DefinedTypeLinkNode {
                    identifier: this.identifier.clone(),
                    program,
                    transforms,
                    plugins,
                })))
            }
            Self::DurationType(this) => {
                let Some(number) = map_required::<IntegerTypeNode>(NodeRef::from(&this.number), f)?
                else {
                    return Ok(None);
                };
                let transforms = map_array::<TransformNode, _>(&this.transforms, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::DurationType(DurationTypeNode {
                    ticks_per_second: this.ticks_per_second.clone(),
                    number,
                    transforms,
                    plugins,
                })))
            }
            Self::EnumType(this) => {
                let variants = map_array::<EnumVariantTypeNode, _>(&this.variants, f)?;
                let Some(size) = map_required::<IntegerTypeNode>(NodeRef::from(&this.size), f)?
                else {
                    return Ok(None);
                };
                let transforms = map_array::<TransformNode, _>(&this.transforms, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::EnumType(EnumTypeNode {
                    variants,
                    size,
                    transforms,
                    plugins,
                })))
            }
            Self::EnumValue(this) => {
                let Some(r#enum) =
                    map_required::<DefinedTypeLinkNode>(NodeRef::from(&this.r#enum), f)?
                else {
                    return Ok(None);
                };
                let value =
                    map_optional::<ValueNode>((*this.value).as_ref().map(NodeRef::from), f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::EnumValue(EnumValueNode {
                    variant: this.variant.clone(),
                    r#enum,
                    value: Box::new(value),
                    plugins,
                })))
            }
            Self::EnumVariantDisplay(this) => {
                let label = map_optional_text(this.label.as_ref(), f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::EnumVariantDisplay(EnumVariantDisplayNode {
                    label,
                    skip_inner_data: this.skip_inner_data.clone(),
                    plugins,
                })))
            }
            Self::EnumVariantType(this) => {
                let docs = map_docs(&this.docs, f)?;
                let data = map_optional::<TypeNode>((*this.data).as_ref().map(NodeRef::from), f)?;
                let display = map_optional::<EnumVariantDisplayNode>(
                    this.display.as_ref().map(NodeRef::from),
                    f,
                )?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::EnumVariantType(EnumVariantTypeNode {
                    identifier: this.identifier.clone(),
                    discriminator: this.discriminator.clone(),
                    docs,
                    data: Box::new(data),
                    display,
                    plugins,
                })))
            }
            Self::Error(this) => {
                let Some(message) = map_text(&this.message, f)? else {
                    return Ok(None);
                };
                let docs = map_docs(&this.docs, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::Error(ErrorNode {
                    identifier: this.identifier.clone(),
                    code: this.code.clone(),
                    message,
                    docs,
                    plugins,
                })))
            }
            Self::Event(this) => {
                let docs = map_docs(&this.docs, f)?;
                let Some(data) = map_required::<TypeNode>(NodeRef::from(&*this.data), f)? else {
                    return Ok(None);
                };
                let discriminators = map_array::<DiscriminatorNode, _>(&this.discriminators, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::Event(EventNode {
                    identifier: this.identifier.clone(),
                    docs,
                    data: Box::new(data),
                    discriminators,
                    plugins,
                })))
            }
            Self::FieldDiscriminator(this) => {
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::FieldDiscriminator(FieldDiscriminatorNode {
                    path: this.path.clone(),
                    offset: this.offset.clone(),
                    plugins,
                })))
            }
            Self::FixedCount(this) => {
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::FixedCount(FixedCountNode {
                    value: this.value.clone(),
                    plugins,
                })))
            }
            Self::FixedPointType(this) => {
                let Some(number) = map_required::<IntegerTypeNode>(NodeRef::from(&this.number), f)?
                else {
                    return Ok(None);
                };
                let display = map_optional::<UnitNumberDisplayNode>(
                    this.display.as_ref().map(NodeRef::from),
                    f,
                )?;
                let transforms = map_array::<TransformNode, _>(&this.transforms, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::FixedPointType(FixedPointTypeNode {
                    scale: this.scale.clone(),
                    base: this.base.clone(),
                    unit: this.unit.clone(),
                    number,
                    display,
                    transforms,
                    plugins,
                })))
            }
            Self::FixedSizeTransform(this) => {
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::FixedSizeTransform(FixedSizeTransformNode {
                    size: this.size.clone(),
                    plugins,
                })))
            }
            Self::FloatType(this) => {
                let display = map_optional::<UnitNumberDisplayNode>(
                    this.display.as_ref().map(NodeRef::from),
                    f,
                )?;
                let transforms = map_array::<TransformNode, _>(&this.transforms, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::FloatType(FloatTypeNode {
                    format: this.format.clone(),
                    endian: this.endian.clone(),
                    unit: this.unit.clone(),
                    display,
                    transforms,
                    plugins,
                })))
            }
            Self::FloatValue(this) => {
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::FloatValue(FloatValueNode {
                    value: this.value.clone(),
                    plugins,
                })))
            }
            Self::HiddenPrefixTransform(this) => {
                let prefix = map_array::<ConstantValueNode, _>(&this.prefix, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::HiddenPrefixTransform(
                    HiddenPrefixTransformNode { prefix, plugins },
                )))
            }
            Self::HiddenSuffixTransform(this) => {
                let suffix = map_array::<ConstantValueNode, _>(&this.suffix, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::HiddenSuffixTransform(
                    HiddenSuffixTransformNode { suffix, plugins },
                )))
            }
            Self::IdentityValue(this) => {
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::IdentityValue(IdentityValueNode { plugins })))
            }
            Self::InjectedValue(this) => {
                let fallback =
                    map_optional::<ValueNode>((*this.fallback).as_ref().map(NodeRef::from), f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::InjectedValue(InjectedValueNode {
                    key: this.key.clone(),
                    fallback: Box::new(fallback),
                    plugins,
                })))
            }
            Self::Instruction(this) => {
                let docs = map_docs(&this.docs, f)?;
                let accounts = map_array::<InstructionAccountNode, _>(&this.accounts, f)?;
                let data = map_optional::<TypeNode>((*this.data).as_ref().map(NodeRef::from), f)?;
                let return_data =
                    map_optional::<TypeNode>((*this.return_data).as_ref().map(NodeRef::from), f)?;
                let remaining_accounts =
                    map_array::<InstructionRemainingAccountsNode, _>(&this.remaining_accounts, f)?;
                let byte_deltas = map_array::<InstructionByteDeltaNode, _>(&this.byte_deltas, f)?;
                let discriminators = map_array::<DiscriminatorNode, _>(&this.discriminators, f)?;
                let status = map_optional::<InstructionStatusNode>(
                    this.status.as_ref().map(NodeRef::from),
                    f,
                )?;
                let sub_instructions = map_array::<InstructionNode, _>(&this.sub_instructions, f)?;
                let provides = map_array::<ProvidedNode, _>(&this.provides, f)?;
                let display = map_optional::<InstructionDisplayNode>(
                    this.display.as_ref().map(NodeRef::from),
                    f,
                )?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::Instruction(InstructionNode {
                    identifier: this.identifier.clone(),
                    docs,
                    optional_account_strategy: this.optional_account_strategy.clone(),
                    accounts,
                    data: Box::new(data),
                    return_data: Box::new(return_data),
                    remaining_accounts,
                    byte_deltas,
                    discriminators,
                    status,
                    sub_instructions,
                    provides,
                    display,
                    plugins,
                })))
            }
            Self::InstructionAccount(this) => {
                let docs = map_docs(&this.docs, f)?;
                let default_value = map_optional::<InstructionInputValueNode>(
                    (*this.default_value).as_ref().map(NodeRef::from),
                    f,
                )?;
                let account_link = map_optional::<AccountLinkNode>(
                    this.account_link.as_ref().map(NodeRef::from),
                    f,
                )?;
                let display = map_optional::<InstructionAccountDisplayNode>(
                    this.display.as_ref().map(NodeRef::from),
                    f,
                )?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::InstructionAccount(InstructionAccountNode {
                    identifier: this.identifier.clone(),
                    is_writable: this.is_writable.clone(),
                    is_signer: this.is_signer.clone(),
                    is_optional: this.is_optional.clone(),
                    docs,
                    default_value: Box::new(default_value),
                    account_link,
                    display,
                    plugins,
                })))
            }
            Self::InstructionAccountDisplay(this) => {
                let label = map_optional_text(this.label.as_ref(), f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::InstructionAccountDisplay(
                    InstructionAccountDisplayNode {
                        label,
                        skip: this.skip.clone(),
                        plugins,
                    },
                )))
            }
            Self::InstructionAccountLink(this) => {
                let instruction = map_optional::<InstructionLinkNode>(
                    this.instruction.as_ref().map(NodeRef::from),
                    f,
                )?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::InstructionAccountLink(
                    InstructionAccountLinkNode {
                        identifier: this.identifier.clone(),
                        instruction,
                        plugins,
                    },
                )))
            }
            Self::InstructionByteDelta(this) => {
                let Some(value) =
                    map_required::<InstructionByteDeltaValue>(NodeRef::from(&*this.value), f)?
                else {
                    return Ok(None);
                };
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::InstructionByteDelta(InstructionByteDeltaNode {
                    with_header: this.with_header.clone(),
                    subtract: this.subtract.clone(),
                    value: Box::new(value),
                    plugins,
                })))
            }
            Self::InstructionDisplay(this) => {
                let intent = map_optional_text(this.intent.as_ref(), f)?;
                let interpolated_intent = map_optional_text(this.interpolated_intent.as_ref(), f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::InstructionDisplay(InstructionDisplayNode {
                    intent,
                    interpolated_intent,
                    plugins,
                })))
            }
            Self::InstructionLink(this) => {
                let program =
                    map_optional::<ProgramLinkNode>(this.program.as_ref().map(NodeRef::from), f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::InstructionLink(InstructionLinkNode {
                    identifier: this.identifier.clone(),
                    program,
                    plugins,
                })))
            }
            Self::InstructionRemainingAccounts(this) => {
                let docs = map_docs(&this.docs, f)?;
                let display = map_optional::<InstructionAccountDisplayNode>(
                    this.display.as_ref().map(NodeRef::from),
                    f,
                )?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::InstructionRemainingAccounts(
                    InstructionRemainingAccountsNode {
                        identifier: this.identifier.clone(),
                        is_optional: this.is_optional.clone(),
                        is_signer: this.is_signer.clone(),
                        is_writable: this.is_writable.clone(),
                        docs,
                        display,
                        plugins,
                    },
                )))
            }
            Self::InstructionStatus(this) => {
                let message = map_optional_text(this.message.as_ref(), f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::InstructionStatus(InstructionStatusNode {
                    lifecycle: this.lifecycle.clone(),
                    message,
                    plugins,
                })))
            }
            Self::IntegerType(this) => {
                let display = map_optional::<NumberDisplayNode>(
                    (*this.display).as_ref().map(NodeRef::from),
                    f,
                )?;
                let transforms = map_array::<TransformNode, _>(&this.transforms, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::IntegerType(IntegerTypeNode {
                    format: this.format.clone(),
                    endian: this.endian.clone(),
                    unit: this.unit.clone(),
                    display: Box::new(display),
                    transforms,
                    plugins,
                })))
            }
            Self::IntegerValue(this) => {
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::IntegerValue(IntegerValueNode {
                    value: this.value.clone(),
                    plugins,
                })))
            }
            Self::MapEntryValue(this) => {
                let Some(key) = map_required::<ValueNode>(NodeRef::from(&*this.key), f)? else {
                    return Ok(None);
                };
                let Some(value) = map_required::<ValueNode>(NodeRef::from(&*this.value), f)? else {
                    return Ok(None);
                };
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::MapEntryValue(MapEntryValueNode {
                    key: Box::new(key),
                    value: Box::new(value),
                    plugins,
                })))
            }
            Self::MapType(this) => {
                let Some(key) = map_required::<TypeNode>(NodeRef::from(&*this.key), f)? else {
                    return Ok(None);
                };
                let Some(value) = map_required::<TypeNode>(NodeRef::from(&*this.value), f)? else {
                    return Ok(None);
                };
                let Some(count) = map_required::<CountNode>(NodeRef::from(&*this.count), f)? else {
                    return Ok(None);
                };
                let transforms = map_array::<TransformNode, _>(&this.transforms, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::MapType(MapTypeNode {
                    key: Box::new(key),
                    value: Box::new(value),
                    count: Box::new(count),
                    transforms,
                    plugins,
                })))
            }
            Self::MapValue(this) => {
                let entries = map_array::<MapEntryValueNode, _>(&this.entries, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::MapValue(MapValueNode { entries, plugins })))
            }
            Self::NoneValue(this) => {
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::NoneValue(NoneValueNode { plugins })))
            }
            Self::OptionType(this) => {
                let Some(item) = map_required::<TypeNode>(NodeRef::from(&*this.item), f)? else {
                    return Ok(None);
                };
                let Some(prefix) = map_required::<IntegerTypeNode>(NodeRef::from(&this.prefix), f)?
                else {
                    return Ok(None);
                };
                let transforms = map_array::<TransformNode, _>(&this.transforms, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::OptionType(OptionTypeNode {
                    fixed: this.fixed.clone(),
                    item: Box::new(item),
                    prefix,
                    transforms,
                    plugins,
                })))
            }
            Self::PayerValue(this) => {
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::PayerValue(PayerValueNode { plugins })))
            }
            Self::Pda(this) => {
                let docs = map_docs(&this.docs, f)?;
                let seeds = map_array::<PdaSeedNode, _>(&this.seeds, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::Pda(PdaNode {
                    identifier: this.identifier.clone(),
                    docs,
                    program_id: this.program_id.clone(),
                    seeds,
                    plugins,
                })))
            }
            Self::PdaLink(this) => {
                let program =
                    map_optional::<ProgramLinkNode>(this.program.as_ref().map(NodeRef::from), f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::PdaLink(PdaLinkNode {
                    identifier: this.identifier.clone(),
                    program,
                    plugins,
                })))
            }
            Self::PdaSeedValue(this) => {
                let Some(value) =
                    map_required::<PdaSeedValueValue>(NodeRef::from(&*this.value), f)?
                else {
                    return Ok(None);
                };
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::PdaSeedValue(PdaSeedValueNode {
                    identifier: this.identifier.clone(),
                    value: Box::new(value),
                    plugins,
                })))
            }
            Self::PdaValue(this) => {
                let Some(pda) = map_required::<PdaValuePda>(NodeRef::from(&*this.pda), f)? else {
                    return Ok(None);
                };
                let seeds = map_array::<PdaSeedValueNode, _>(&this.seeds, f)?;
                let program_id = map_optional::<PdaValueProgramId>(
                    (*this.program_id).as_ref().map(NodeRef::from),
                    f,
                )?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::PdaValue(PdaValueNode {
                    pda: Box::new(pda),
                    seeds,
                    program_id: Box::new(program_id),
                    plugins,
                })))
            }
            Self::Plugin(this) => {
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::Plugin(PluginNode {
                    namespace: this.namespace.clone(),
                    payload: this.payload.clone(),
                    plugins,
                })))
            }
            Self::PostOffsetTransform(this) => {
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::PostOffsetTransform(PostOffsetTransformNode {
                    offset: this.offset.clone(),
                    strategy: this.strategy.clone(),
                    plugins,
                })))
            }
            Self::PrefixedCount(this) => {
                let Some(prefix) = map_required::<IntegerTypeNode>(NodeRef::from(&this.prefix), f)?
                else {
                    return Ok(None);
                };
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::PrefixedCount(PrefixedCountNode {
                    prefix,
                    plugins,
                })))
            }
            Self::PreOffsetTransform(this) => {
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::PreOffsetTransform(PreOffsetTransformNode {
                    offset: this.offset.clone(),
                    strategy: this.strategy.clone(),
                    plugins,
                })))
            }
            Self::Program(this) => {
                let docs = map_docs(&this.docs, f)?;
                let accounts = map_array::<AccountNode, _>(&this.accounts, f)?;
                let instructions = map_array::<InstructionNode, _>(&this.instructions, f)?;
                let defined_types = map_array::<DefinedTypeNode, _>(&this.defined_types, f)?;
                let pdas = map_array::<PdaNode, _>(&this.pdas, f)?;
                let events = map_array::<EventNode, _>(&this.events, f)?;
                let errors = map_array::<ErrorNode, _>(&this.errors, f)?;
                let constants = map_array::<ConstantNode, _>(&this.constants, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::Program(ProgramNode {
                    identifier: this.identifier.clone(),
                    public_key: this.public_key.clone(),
                    version: this.version.clone(),
                    docs,
                    accounts,
                    instructions,
                    defined_types,
                    pdas,
                    events,
                    errors,
                    constants,
                    plugins,
                })))
            }
            Self::ProgramIdValue(this) => {
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::ProgramIdValue(ProgramIdValueNode { plugins })))
            }
            Self::ProgramLink(this) => {
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::ProgramLink(ProgramLinkNode {
                    identifier: this.identifier.clone(),
                    plugins,
                })))
            }
            Self::Provided(this) => {
                let Some(node) = f(NodeRef::from(&*this.node))? else {
                    return Ok(None);
                };
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::Provided(ProvidedNode {
                    identifier: this.identifier.clone(),
                    node: Box::new(node),
                    plugins,
                })))
            }
            Self::PublicKeyType(this) => {
                let account_link = map_optional::<AccountLinkNode>(
                    this.account_link.as_ref().map(NodeRef::from),
                    f,
                )?;
                let transforms = map_array::<TransformNode, _>(&this.transforms, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::PublicKeyType(PublicKeyTypeNode {
                    account_link,
                    transforms,
                    plugins,
                })))
            }
            Self::PublicKeyValue(this) => {
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::PublicKeyValue(PublicKeyValueNode {
                    public_key: this.public_key.clone(),
                    identifier: this.identifier.clone(),
                    plugins,
                })))
            }
            Self::RemainderCount(this) => {
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::RemainderCount(RemainderCountNode { plugins })))
            }
            Self::RemainderOptionType(this) => {
                let Some(item) = map_required::<TypeNode>(NodeRef::from(&*this.item), f)? else {
                    return Ok(None);
                };
                let transforms = map_array::<TransformNode, _>(&this.transforms, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::RemainderOptionType(RemainderOptionTypeNode {
                    item: Box::new(item),
                    transforms,
                    plugins,
                })))
            }
            Self::Root(this) => {
                let Some(program) = map_required::<ProgramNode>(NodeRef::from(&this.program), f)?
                else {
                    return Ok(None);
                };
                let additional_programs =
                    map_array::<ProgramNode, _>(&this.additional_programs, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::Root(RootNode {
                    standard: this.standard.clone(),
                    version: this.version.clone(),
                    program,
                    additional_programs,
                    plugins,
                })))
            }
            Self::SentinelCount(this) => {
                let Some(sentinel) =
                    map_required::<ConstantValueNode>(NodeRef::from(&this.sentinel), f)?
                else {
                    return Ok(None);
                };
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::SentinelCount(SentinelCountNode {
                    strategy: this.strategy.clone(),
                    sentinel,
                    plugins,
                })))
            }
            Self::SentinelTransform(this) => {
                let Some(sentinel) =
                    map_required::<ConstantValueNode>(NodeRef::from(&this.sentinel), f)?
                else {
                    return Ok(None);
                };
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::SentinelTransform(SentinelTransformNode {
                    sentinel,
                    plugins,
                })))
            }
            Self::SetType(this) => {
                let Some(item) = map_required::<TypeNode>(NodeRef::from(&*this.item), f)? else {
                    return Ok(None);
                };
                let Some(count) = map_required::<CountNode>(NodeRef::from(&*this.count), f)? else {
                    return Ok(None);
                };
                let transforms = map_array::<TransformNode, _>(&this.transforms, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::SetType(SetTypeNode {
                    item: Box::new(item),
                    count: Box::new(count),
                    transforms,
                    plugins,
                })))
            }
            Self::SetValue(this) => {
                let items = map_array::<ValueNode, _>(&this.items, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::SetValue(SetValueNode { items, plugins })))
            }
            Self::SizeDiscriminator(this) => {
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::SizeDiscriminator(SizeDiscriminatorNode {
                    size: this.size.clone(),
                    plugins,
                })))
            }
            Self::SizePrefixTransform(this) => {
                let Some(prefix) = map_required::<IntegerTypeNode>(NodeRef::from(&this.prefix), f)?
                else {
                    return Ok(None);
                };
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::SizePrefixTransform(SizePrefixTransformNode {
                    prefix,
                    plugins,
                })))
            }
            Self::SomeValue(this) => {
                let Some(value) = map_required::<ValueNode>(NodeRef::from(&*this.value), f)? else {
                    return Ok(None);
                };
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::SomeValue(SomeValueNode {
                    value: Box::new(value),
                    plugins,
                })))
            }
            Self::StringDisplay(this) => {
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::StringDisplay(StringDisplayNode {
                    slice_start: this.slice_start.clone(),
                    slice_end: this.slice_end.clone(),
                    plugins,
                })))
            }
            Self::StringType(this) => {
                let display =
                    map_optional::<StringDisplayNode>(this.display.as_ref().map(NodeRef::from), f)?;
                let transforms = map_array::<TransformNode, _>(&this.transforms, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::StringType(StringTypeNode {
                    encoding: this.encoding.clone(),
                    display,
                    transforms,
                    plugins,
                })))
            }
            Self::StringValue(this) => {
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::StringValue(StringValueNode {
                    string: this.string.clone(),
                    plugins,
                })))
            }
            Self::StructFieldDisplay(this) => {
                let label = map_optional_text(this.label.as_ref(), f)?;
                let flatten_prefix = map_optional_text(this.flatten_prefix.as_ref(), f)?;
                let value_labels = map_array::<ValueLabelDisplayNode, _>(&this.value_labels, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::StructFieldDisplay(StructFieldDisplayNode {
                    label,
                    skip: this.skip.clone(),
                    flatten: this.flatten.clone(),
                    flatten_prefix,
                    value_labels,
                    plugins,
                })))
            }
            Self::StructFieldType(this) => {
                let docs = map_docs(&this.docs, f)?;
                let Some(r#type) = map_required::<TypeNode>(NodeRef::from(&*this.r#type), f)?
                else {
                    return Ok(None);
                };
                let default_value = map_optional::<ValueNode>(
                    (*this.default_value).as_ref().map(NodeRef::from),
                    f,
                )?;
                let display = map_optional::<StructFieldDisplayNode>(
                    this.display.as_ref().map(NodeRef::from),
                    f,
                )?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::StructFieldType(StructFieldTypeNode {
                    identifier: this.identifier.clone(),
                    default_value_strategy: this.default_value_strategy.clone(),
                    docs,
                    r#type: Box::new(r#type),
                    default_value: Box::new(default_value),
                    display,
                    plugins,
                })))
            }
            Self::StructFieldValue(this) => {
                let Some(value) = map_required::<ValueNode>(NodeRef::from(&*this.value), f)? else {
                    return Ok(None);
                };
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::StructFieldValue(StructFieldValueNode {
                    identifier: this.identifier.clone(),
                    value: Box::new(value),
                    plugins,
                })))
            }
            Self::StructType(this) => {
                let fields = map_array::<StructFieldTypeNode, _>(&this.fields, f)?;
                let transforms = map_array::<TransformNode, _>(&this.transforms, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::StructType(StructTypeNode {
                    fields,
                    transforms,
                    plugins,
                })))
            }
            Self::StructValue(this) => {
                let fields = map_array::<StructFieldValueNode, _>(&this.fields, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::StructValue(StructValueNode { fields, plugins })))
            }
            Self::Text(this) => {
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::Text(TextNode {
                    content: this.content.clone(),
                    plugins,
                })))
            }
            Self::TupleType(this) => {
                let items = map_array::<TypeNode, _>(&this.items, f)?;
                let transforms = map_array::<TransformNode, _>(&this.transforms, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::TupleType(TupleTypeNode {
                    items,
                    transforms,
                    plugins,
                })))
            }
            Self::TupleValue(this) => {
                let items = map_array::<ValueNode, _>(&this.items, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::TupleValue(TupleValueNode { items, plugins })))
            }
            Self::UnitNumberDisplay(this) => {
                let Some(unit) =
                    map_required::<InjectableUnitValueNode>(NodeRef::from(&*this.unit), f)?
                else {
                    return Ok(None);
                };
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::UnitNumberDisplay(UnitNumberDisplayNode {
                    unit: Box::new(unit),
                    plugins,
                })))
            }
            Self::ValueLabelDisplay(this) => {
                let Some(label) = map_text(&this.label, f)? else {
                    return Ok(None);
                };
                let Some(value) = map_required::<ValueNode>(NodeRef::from(&*this.value), f)? else {
                    return Ok(None);
                };
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::ValueLabelDisplay(ValueLabelDisplayNode {
                    label,
                    value: Box::new(value),
                    plugins,
                })))
            }
            Self::VariablePdaSeed(this) => {
                let docs = map_docs(&this.docs, f)?;
                let Some(r#type) = map_required::<TypeNode>(NodeRef::from(&*this.r#type), f)?
                else {
                    return Ok(None);
                };
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::VariablePdaSeed(VariablePdaSeedNode {
                    identifier: this.identifier.clone(),
                    docs,
                    r#type: Box::new(r#type),
                    plugins,
                })))
            }
            Self::ZeroableOptionType(this) => {
                let Some(item) = map_required::<TypeNode>(NodeRef::from(&*this.item), f)? else {
                    return Ok(None);
                };
                let zero_value = map_optional::<ConstantValueNode>(
                    this.zero_value.as_ref().map(NodeRef::from),
                    f,
                )?;
                let transforms = map_array::<TransformNode, _>(&this.transforms, f)?;
                let plugins = map_array::<PluginNode, _>(&this.plugins, f)?;
                Ok(Some(Node::ZeroableOptionType(ZeroableOptionTypeNode {
                    item: Box::new(item),
                    zero_value,
                    transforms,
                    plugins,
                })))
            }
        }
    }
}

impl HasKind for NodeRef<'_> {
    fn kind(&self) -> &'static str {
        match self {
            Self::Account(node) => node.kind(),
            Self::AccountBumpValue(node) => node.kind(),
            Self::AccountDataValue(node) => node.kind(),
            Self::AccountLink(node) => node.kind(),
            Self::AccountValue(node) => node.kind(),
            Self::AmountNumberDisplay(node) => node.kind(),
            Self::ArrayType(node) => node.kind(),
            Self::ArrayValue(node) => node.kind(),
            Self::BooleanType(node) => node.kind(),
            Self::BooleanValue(node) => node.kind(),
            Self::BytesType(node) => node.kind(),
            Self::BytesValue(node) => node.kind(),
            Self::ConditionalValue(node) => node.kind(),
            Self::Constant(node) => node.kind(),
            Self::ConstantDiscriminator(node) => node.kind(),
            Self::ConstantPdaSeed(node) => node.kind(),
            Self::ConstantValue(node) => node.kind(),
            Self::DataValue(node) => node.kind(),
            Self::DateTimeType(node) => node.kind(),
            Self::DefinedType(node) => node.kind(),
            Self::DefinedTypeLink(node) => node.kind(),
            Self::DurationType(node) => node.kind(),
            Self::EnumType(node) => node.kind(),
            Self::EnumValue(node) => node.kind(),
            Self::EnumVariantDisplay(node) => node.kind(),
            Self::EnumVariantType(node) => node.kind(),
            Self::Error(node) => node.kind(),
            Self::Event(node) => node.kind(),
            Self::FieldDiscriminator(node) => node.kind(),
            Self::FixedCount(node) => node.kind(),
            Self::FixedPointType(node) => node.kind(),
            Self::FixedSizeTransform(node) => node.kind(),
            Self::FloatType(node) => node.kind(),
            Self::FloatValue(node) => node.kind(),
            Self::HiddenPrefixTransform(node) => node.kind(),
            Self::HiddenSuffixTransform(node) => node.kind(),
            Self::IdentityValue(node) => node.kind(),
            Self::InjectedValue(node) => node.kind(),
            Self::Instruction(node) => node.kind(),
            Self::InstructionAccount(node) => node.kind(),
            Self::InstructionAccountDisplay(node) => node.kind(),
            Self::InstructionAccountLink(node) => node.kind(),
            Self::InstructionByteDelta(node) => node.kind(),
            Self::InstructionDisplay(node) => node.kind(),
            Self::InstructionLink(node) => node.kind(),
            Self::InstructionRemainingAccounts(node) => node.kind(),
            Self::InstructionStatus(node) => node.kind(),
            Self::IntegerType(node) => node.kind(),
            Self::IntegerValue(node) => node.kind(),
            Self::MapEntryValue(node) => node.kind(),
            Self::MapType(node) => node.kind(),
            Self::MapValue(node) => node.kind(),
            Self::NoneValue(node) => node.kind(),
            Self::OptionType(node) => node.kind(),
            Self::PayerValue(node) => node.kind(),
            Self::Pda(node) => node.kind(),
            Self::PdaLink(node) => node.kind(),
            Self::PdaSeedValue(node) => node.kind(),
            Self::PdaValue(node) => node.kind(),
            Self::Plugin(node) => node.kind(),
            Self::PostOffsetTransform(node) => node.kind(),
            Self::PrefixedCount(node) => node.kind(),
            Self::PreOffsetTransform(node) => node.kind(),
            Self::Program(node) => node.kind(),
            Self::ProgramIdValue(node) => node.kind(),
            Self::ProgramLink(node) => node.kind(),
            Self::Provided(node) => node.kind(),
            Self::PublicKeyType(node) => node.kind(),
            Self::PublicKeyValue(node) => node.kind(),
            Self::RemainderCount(node) => node.kind(),
            Self::RemainderOptionType(node) => node.kind(),
            Self::Root(node) => node.kind(),
            Self::SentinelCount(node) => node.kind(),
            Self::SentinelTransform(node) => node.kind(),
            Self::SetType(node) => node.kind(),
            Self::SetValue(node) => node.kind(),
            Self::SizeDiscriminator(node) => node.kind(),
            Self::SizePrefixTransform(node) => node.kind(),
            Self::SomeValue(node) => node.kind(),
            Self::StringDisplay(node) => node.kind(),
            Self::StringType(node) => node.kind(),
            Self::StringValue(node) => node.kind(),
            Self::StructFieldDisplay(node) => node.kind(),
            Self::StructFieldType(node) => node.kind(),
            Self::StructFieldValue(node) => node.kind(),
            Self::StructType(node) => node.kind(),
            Self::StructValue(node) => node.kind(),
            Self::Text(node) => node.kind(),
            Self::TupleType(node) => node.kind(),
            Self::TupleValue(node) => node.kind(),
            Self::UnitNumberDisplay(node) => node.kind(),
            Self::ValueLabelDisplay(node) => node.kind(),
            Self::VariablePdaSeed(node) => node.kind(),
            Self::ZeroableOptionType(node) => node.kind(),
        }
    }
}

impl TryFrom<Node> for AccountNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Account(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "accountNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a AccountNode> for NodeRef<'a> {
    fn from(node: &'a AccountNode) -> Self {
        Self::Account(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a AccountNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::Account(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "accountNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for AccountBumpValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::AccountBumpValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "accountBumpValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a AccountBumpValueNode> for NodeRef<'a> {
    fn from(node: &'a AccountBumpValueNode) -> Self {
        Self::AccountBumpValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a AccountBumpValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::AccountBumpValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "accountBumpValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for AccountDataValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::AccountDataValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "accountDataValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a AccountDataValueNode> for NodeRef<'a> {
    fn from(node: &'a AccountDataValueNode) -> Self {
        Self::AccountDataValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a AccountDataValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::AccountDataValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "accountDataValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for AccountLinkNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::AccountLink(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "accountLinkNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a AccountLinkNode> for NodeRef<'a> {
    fn from(node: &'a AccountLinkNode) -> Self {
        Self::AccountLink(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a AccountLinkNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::AccountLink(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "accountLinkNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for AccountValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::AccountValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "accountValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a AccountValueNode> for NodeRef<'a> {
    fn from(node: &'a AccountValueNode) -> Self {
        Self::AccountValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a AccountValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::AccountValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "accountValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for AmountNumberDisplayNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::AmountNumberDisplay(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "amountNumberDisplayNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a AmountNumberDisplayNode> for NodeRef<'a> {
    fn from(node: &'a AmountNumberDisplayNode) -> Self {
        Self::AmountNumberDisplay(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a AmountNumberDisplayNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::AmountNumberDisplay(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "amountNumberDisplayNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for ArrayTypeNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::ArrayType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "arrayTypeNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a ArrayTypeNode> for NodeRef<'a> {
    fn from(node: &'a ArrayTypeNode) -> Self {
        Self::ArrayType(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a ArrayTypeNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::ArrayType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "arrayTypeNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for ArrayValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::ArrayValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "arrayValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a ArrayValueNode> for NodeRef<'a> {
    fn from(node: &'a ArrayValueNode) -> Self {
        Self::ArrayValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a ArrayValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::ArrayValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "arrayValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for BooleanTypeNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::BooleanType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "booleanTypeNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a BooleanTypeNode> for NodeRef<'a> {
    fn from(node: &'a BooleanTypeNode) -> Self {
        Self::BooleanType(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a BooleanTypeNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::BooleanType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "booleanTypeNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for BooleanValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::BooleanValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "booleanValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a BooleanValueNode> for NodeRef<'a> {
    fn from(node: &'a BooleanValueNode) -> Self {
        Self::BooleanValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a BooleanValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::BooleanValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "booleanValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for BytesTypeNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::BytesType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "bytesTypeNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a BytesTypeNode> for NodeRef<'a> {
    fn from(node: &'a BytesTypeNode) -> Self {
        Self::BytesType(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a BytesTypeNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::BytesType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "bytesTypeNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for BytesValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::BytesValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "bytesValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a BytesValueNode> for NodeRef<'a> {
    fn from(node: &'a BytesValueNode) -> Self {
        Self::BytesValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a BytesValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::BytesValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "bytesValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for ConditionalValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::ConditionalValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "conditionalValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a ConditionalValueNode> for NodeRef<'a> {
    fn from(node: &'a ConditionalValueNode) -> Self {
        Self::ConditionalValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a ConditionalValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::ConditionalValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "conditionalValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for ConstantNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Constant(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "constantNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a ConstantNode> for NodeRef<'a> {
    fn from(node: &'a ConstantNode) -> Self {
        Self::Constant(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a ConstantNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::Constant(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "constantNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for ConstantDiscriminatorNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::ConstantDiscriminator(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "constantDiscriminatorNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a ConstantDiscriminatorNode> for NodeRef<'a> {
    fn from(node: &'a ConstantDiscriminatorNode) -> Self {
        Self::ConstantDiscriminator(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a ConstantDiscriminatorNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::ConstantDiscriminator(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "constantDiscriminatorNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for ConstantPdaSeedNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::ConstantPdaSeed(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "constantPdaSeedNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a ConstantPdaSeedNode> for NodeRef<'a> {
    fn from(node: &'a ConstantPdaSeedNode) -> Self {
        Self::ConstantPdaSeed(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a ConstantPdaSeedNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::ConstantPdaSeed(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "constantPdaSeedNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for ConstantValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::ConstantValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "constantValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a ConstantValueNode> for NodeRef<'a> {
    fn from(node: &'a ConstantValueNode) -> Self {
        Self::ConstantValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a ConstantValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::ConstantValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "constantValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for DataValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::DataValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "dataValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a DataValueNode> for NodeRef<'a> {
    fn from(node: &'a DataValueNode) -> Self {
        Self::DataValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a DataValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::DataValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "dataValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for DateTimeTypeNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::DateTimeType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "dateTimeTypeNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a DateTimeTypeNode> for NodeRef<'a> {
    fn from(node: &'a DateTimeTypeNode) -> Self {
        Self::DateTimeType(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a DateTimeTypeNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::DateTimeType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "dateTimeTypeNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for DefinedTypeNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::DefinedType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "definedTypeNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a DefinedTypeNode> for NodeRef<'a> {
    fn from(node: &'a DefinedTypeNode) -> Self {
        Self::DefinedType(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a DefinedTypeNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::DefinedType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "definedTypeNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for DefinedTypeLinkNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::DefinedTypeLink(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "definedTypeLinkNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a DefinedTypeLinkNode> for NodeRef<'a> {
    fn from(node: &'a DefinedTypeLinkNode) -> Self {
        Self::DefinedTypeLink(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a DefinedTypeLinkNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::DefinedTypeLink(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "definedTypeLinkNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for DurationTypeNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::DurationType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "durationTypeNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a DurationTypeNode> for NodeRef<'a> {
    fn from(node: &'a DurationTypeNode) -> Self {
        Self::DurationType(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a DurationTypeNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::DurationType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "durationTypeNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for EnumTypeNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::EnumType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "enumTypeNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a EnumTypeNode> for NodeRef<'a> {
    fn from(node: &'a EnumTypeNode) -> Self {
        Self::EnumType(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a EnumTypeNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::EnumType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "enumTypeNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for EnumValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::EnumValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "enumValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a EnumValueNode> for NodeRef<'a> {
    fn from(node: &'a EnumValueNode) -> Self {
        Self::EnumValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a EnumValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::EnumValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "enumValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for EnumVariantDisplayNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::EnumVariantDisplay(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "enumVariantDisplayNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a EnumVariantDisplayNode> for NodeRef<'a> {
    fn from(node: &'a EnumVariantDisplayNode) -> Self {
        Self::EnumVariantDisplay(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a EnumVariantDisplayNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::EnumVariantDisplay(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "enumVariantDisplayNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for EnumVariantTypeNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::EnumVariantType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "enumVariantTypeNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a EnumVariantTypeNode> for NodeRef<'a> {
    fn from(node: &'a EnumVariantTypeNode) -> Self {
        Self::EnumVariantType(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a EnumVariantTypeNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::EnumVariantType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "enumVariantTypeNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for ErrorNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Error(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "errorNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a ErrorNode> for NodeRef<'a> {
    fn from(node: &'a ErrorNode) -> Self {
        Self::Error(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a ErrorNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::Error(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "errorNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for EventNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Event(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "eventNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a EventNode> for NodeRef<'a> {
    fn from(node: &'a EventNode) -> Self {
        Self::Event(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a EventNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::Event(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "eventNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for FieldDiscriminatorNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::FieldDiscriminator(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "fieldDiscriminatorNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a FieldDiscriminatorNode> for NodeRef<'a> {
    fn from(node: &'a FieldDiscriminatorNode) -> Self {
        Self::FieldDiscriminator(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a FieldDiscriminatorNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::FieldDiscriminator(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "fieldDiscriminatorNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for FixedCountNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::FixedCount(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "fixedCountNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a FixedCountNode> for NodeRef<'a> {
    fn from(node: &'a FixedCountNode) -> Self {
        Self::FixedCount(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a FixedCountNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::FixedCount(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "fixedCountNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for FixedPointTypeNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::FixedPointType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "fixedPointTypeNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a FixedPointTypeNode> for NodeRef<'a> {
    fn from(node: &'a FixedPointTypeNode) -> Self {
        Self::FixedPointType(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a FixedPointTypeNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::FixedPointType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "fixedPointTypeNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for FixedSizeTransformNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::FixedSizeTransform(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "fixedSizeTransformNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a FixedSizeTransformNode> for NodeRef<'a> {
    fn from(node: &'a FixedSizeTransformNode) -> Self {
        Self::FixedSizeTransform(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a FixedSizeTransformNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::FixedSizeTransform(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "fixedSizeTransformNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for FloatTypeNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::FloatType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "floatTypeNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a FloatTypeNode> for NodeRef<'a> {
    fn from(node: &'a FloatTypeNode) -> Self {
        Self::FloatType(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a FloatTypeNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::FloatType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "floatTypeNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for FloatValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::FloatValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "floatValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a FloatValueNode> for NodeRef<'a> {
    fn from(node: &'a FloatValueNode) -> Self {
        Self::FloatValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a FloatValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::FloatValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "floatValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for HiddenPrefixTransformNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::HiddenPrefixTransform(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "hiddenPrefixTransformNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a HiddenPrefixTransformNode> for NodeRef<'a> {
    fn from(node: &'a HiddenPrefixTransformNode) -> Self {
        Self::HiddenPrefixTransform(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a HiddenPrefixTransformNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::HiddenPrefixTransform(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "hiddenPrefixTransformNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for HiddenSuffixTransformNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::HiddenSuffixTransform(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "hiddenSuffixTransformNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a HiddenSuffixTransformNode> for NodeRef<'a> {
    fn from(node: &'a HiddenSuffixTransformNode) -> Self {
        Self::HiddenSuffixTransform(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a HiddenSuffixTransformNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::HiddenSuffixTransform(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "hiddenSuffixTransformNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for IdentityValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::IdentityValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "identityValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a IdentityValueNode> for NodeRef<'a> {
    fn from(node: &'a IdentityValueNode) -> Self {
        Self::IdentityValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a IdentityValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::IdentityValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "identityValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for InjectedValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::InjectedValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "injectedValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a InjectedValueNode> for NodeRef<'a> {
    fn from(node: &'a InjectedValueNode) -> Self {
        Self::InjectedValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a InjectedValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::InjectedValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "injectedValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for InstructionNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Instruction(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "instructionNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a InstructionNode> for NodeRef<'a> {
    fn from(node: &'a InstructionNode) -> Self {
        Self::Instruction(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a InstructionNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::Instruction(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "instructionNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for InstructionAccountNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::InstructionAccount(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "instructionAccountNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a InstructionAccountNode> for NodeRef<'a> {
    fn from(node: &'a InstructionAccountNode) -> Self {
        Self::InstructionAccount(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a InstructionAccountNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::InstructionAccount(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "instructionAccountNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for InstructionAccountDisplayNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::InstructionAccountDisplay(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "instructionAccountDisplayNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a InstructionAccountDisplayNode> for NodeRef<'a> {
    fn from(node: &'a InstructionAccountDisplayNode) -> Self {
        Self::InstructionAccountDisplay(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a InstructionAccountDisplayNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::InstructionAccountDisplay(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "instructionAccountDisplayNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for InstructionAccountLinkNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::InstructionAccountLink(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "instructionAccountLinkNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a InstructionAccountLinkNode> for NodeRef<'a> {
    fn from(node: &'a InstructionAccountLinkNode) -> Self {
        Self::InstructionAccountLink(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a InstructionAccountLinkNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::InstructionAccountLink(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "instructionAccountLinkNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for InstructionByteDeltaNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::InstructionByteDelta(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "instructionByteDeltaNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a InstructionByteDeltaNode> for NodeRef<'a> {
    fn from(node: &'a InstructionByteDeltaNode) -> Self {
        Self::InstructionByteDelta(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a InstructionByteDeltaNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::InstructionByteDelta(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "instructionByteDeltaNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for InstructionDisplayNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::InstructionDisplay(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "instructionDisplayNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a InstructionDisplayNode> for NodeRef<'a> {
    fn from(node: &'a InstructionDisplayNode) -> Self {
        Self::InstructionDisplay(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a InstructionDisplayNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::InstructionDisplay(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "instructionDisplayNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for InstructionLinkNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::InstructionLink(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "instructionLinkNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a InstructionLinkNode> for NodeRef<'a> {
    fn from(node: &'a InstructionLinkNode) -> Self {
        Self::InstructionLink(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a InstructionLinkNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::InstructionLink(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "instructionLinkNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for InstructionRemainingAccountsNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::InstructionRemainingAccounts(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "instructionRemainingAccountsNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a InstructionRemainingAccountsNode> for NodeRef<'a> {
    fn from(node: &'a InstructionRemainingAccountsNode) -> Self {
        Self::InstructionRemainingAccounts(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a InstructionRemainingAccountsNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::InstructionRemainingAccounts(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "instructionRemainingAccountsNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for InstructionStatusNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::InstructionStatus(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "instructionStatusNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a InstructionStatusNode> for NodeRef<'a> {
    fn from(node: &'a InstructionStatusNode) -> Self {
        Self::InstructionStatus(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a InstructionStatusNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::InstructionStatus(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "instructionStatusNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for IntegerTypeNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::IntegerType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "integerTypeNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a IntegerTypeNode> for NodeRef<'a> {
    fn from(node: &'a IntegerTypeNode) -> Self {
        Self::IntegerType(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a IntegerTypeNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::IntegerType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "integerTypeNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for IntegerValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::IntegerValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "integerValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a IntegerValueNode> for NodeRef<'a> {
    fn from(node: &'a IntegerValueNode) -> Self {
        Self::IntegerValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a IntegerValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::IntegerValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "integerValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for MapEntryValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::MapEntryValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "mapEntryValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a MapEntryValueNode> for NodeRef<'a> {
    fn from(node: &'a MapEntryValueNode) -> Self {
        Self::MapEntryValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a MapEntryValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::MapEntryValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "mapEntryValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for MapTypeNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::MapType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "mapTypeNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a MapTypeNode> for NodeRef<'a> {
    fn from(node: &'a MapTypeNode) -> Self {
        Self::MapType(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a MapTypeNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::MapType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "mapTypeNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for MapValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::MapValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "mapValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a MapValueNode> for NodeRef<'a> {
    fn from(node: &'a MapValueNode) -> Self {
        Self::MapValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a MapValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::MapValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "mapValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for NoneValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::NoneValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "noneValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a NoneValueNode> for NodeRef<'a> {
    fn from(node: &'a NoneValueNode) -> Self {
        Self::NoneValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a NoneValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::NoneValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "noneValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for OptionTypeNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::OptionType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "optionTypeNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a OptionTypeNode> for NodeRef<'a> {
    fn from(node: &'a OptionTypeNode) -> Self {
        Self::OptionType(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a OptionTypeNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::OptionType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "optionTypeNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for PayerValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::PayerValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "payerValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a PayerValueNode> for NodeRef<'a> {
    fn from(node: &'a PayerValueNode) -> Self {
        Self::PayerValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a PayerValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::PayerValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "payerValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for PdaNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Pda(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "pdaNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a PdaNode> for NodeRef<'a> {
    fn from(node: &'a PdaNode) -> Self {
        Self::Pda(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a PdaNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::Pda(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "pdaNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for PdaLinkNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::PdaLink(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "pdaLinkNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a PdaLinkNode> for NodeRef<'a> {
    fn from(node: &'a PdaLinkNode) -> Self {
        Self::PdaLink(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a PdaLinkNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::PdaLink(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "pdaLinkNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for PdaSeedValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::PdaSeedValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "pdaSeedValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a PdaSeedValueNode> for NodeRef<'a> {
    fn from(node: &'a PdaSeedValueNode) -> Self {
        Self::PdaSeedValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a PdaSeedValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::PdaSeedValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "pdaSeedValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for PdaValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::PdaValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "pdaValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a PdaValueNode> for NodeRef<'a> {
    fn from(node: &'a PdaValueNode) -> Self {
        Self::PdaValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a PdaValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::PdaValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "pdaValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for PluginNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Plugin(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "pluginNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a PluginNode> for NodeRef<'a> {
    fn from(node: &'a PluginNode) -> Self {
        Self::Plugin(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a PluginNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::Plugin(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "pluginNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for PostOffsetTransformNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::PostOffsetTransform(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "postOffsetTransformNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a PostOffsetTransformNode> for NodeRef<'a> {
    fn from(node: &'a PostOffsetTransformNode) -> Self {
        Self::PostOffsetTransform(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a PostOffsetTransformNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::PostOffsetTransform(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "postOffsetTransformNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for PrefixedCountNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::PrefixedCount(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "prefixedCountNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a PrefixedCountNode> for NodeRef<'a> {
    fn from(node: &'a PrefixedCountNode) -> Self {
        Self::PrefixedCount(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a PrefixedCountNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::PrefixedCount(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "prefixedCountNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for PreOffsetTransformNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::PreOffsetTransform(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "preOffsetTransformNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a PreOffsetTransformNode> for NodeRef<'a> {
    fn from(node: &'a PreOffsetTransformNode) -> Self {
        Self::PreOffsetTransform(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a PreOffsetTransformNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::PreOffsetTransform(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "preOffsetTransformNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for ProgramNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Program(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "programNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a ProgramNode> for NodeRef<'a> {
    fn from(node: &'a ProgramNode) -> Self {
        Self::Program(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a ProgramNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::Program(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "programNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for ProgramIdValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::ProgramIdValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "programIdValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a ProgramIdValueNode> for NodeRef<'a> {
    fn from(node: &'a ProgramIdValueNode) -> Self {
        Self::ProgramIdValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a ProgramIdValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::ProgramIdValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "programIdValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for ProgramLinkNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::ProgramLink(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "programLinkNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a ProgramLinkNode> for NodeRef<'a> {
    fn from(node: &'a ProgramLinkNode) -> Self {
        Self::ProgramLink(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a ProgramLinkNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::ProgramLink(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "programLinkNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for ProvidedNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Provided(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "providedNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a ProvidedNode> for NodeRef<'a> {
    fn from(node: &'a ProvidedNode) -> Self {
        Self::Provided(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a ProvidedNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::Provided(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "providedNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for PublicKeyTypeNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::PublicKeyType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "publicKeyTypeNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a PublicKeyTypeNode> for NodeRef<'a> {
    fn from(node: &'a PublicKeyTypeNode) -> Self {
        Self::PublicKeyType(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a PublicKeyTypeNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::PublicKeyType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "publicKeyTypeNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for PublicKeyValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::PublicKeyValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "publicKeyValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a PublicKeyValueNode> for NodeRef<'a> {
    fn from(node: &'a PublicKeyValueNode) -> Self {
        Self::PublicKeyValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a PublicKeyValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::PublicKeyValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "publicKeyValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for RemainderCountNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::RemainderCount(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "remainderCountNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a RemainderCountNode> for NodeRef<'a> {
    fn from(node: &'a RemainderCountNode) -> Self {
        Self::RemainderCount(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a RemainderCountNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::RemainderCount(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "remainderCountNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for RemainderOptionTypeNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::RemainderOptionType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "remainderOptionTypeNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a RemainderOptionTypeNode> for NodeRef<'a> {
    fn from(node: &'a RemainderOptionTypeNode) -> Self {
        Self::RemainderOptionType(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a RemainderOptionTypeNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::RemainderOptionType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "remainderOptionTypeNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for RootNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Root(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "rootNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a RootNode> for NodeRef<'a> {
    fn from(node: &'a RootNode) -> Self {
        Self::Root(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a RootNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::Root(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "rootNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for SentinelCountNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::SentinelCount(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "sentinelCountNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a SentinelCountNode> for NodeRef<'a> {
    fn from(node: &'a SentinelCountNode) -> Self {
        Self::SentinelCount(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a SentinelCountNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::SentinelCount(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "sentinelCountNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for SentinelTransformNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::SentinelTransform(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "sentinelTransformNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a SentinelTransformNode> for NodeRef<'a> {
    fn from(node: &'a SentinelTransformNode) -> Self {
        Self::SentinelTransform(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a SentinelTransformNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::SentinelTransform(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "sentinelTransformNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for SetTypeNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::SetType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "setTypeNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a SetTypeNode> for NodeRef<'a> {
    fn from(node: &'a SetTypeNode) -> Self {
        Self::SetType(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a SetTypeNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::SetType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "setTypeNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for SetValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::SetValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "setValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a SetValueNode> for NodeRef<'a> {
    fn from(node: &'a SetValueNode) -> Self {
        Self::SetValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a SetValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::SetValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "setValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for SizeDiscriminatorNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::SizeDiscriminator(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "sizeDiscriminatorNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a SizeDiscriminatorNode> for NodeRef<'a> {
    fn from(node: &'a SizeDiscriminatorNode) -> Self {
        Self::SizeDiscriminator(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a SizeDiscriminatorNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::SizeDiscriminator(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "sizeDiscriminatorNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for SizePrefixTransformNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::SizePrefixTransform(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "sizePrefixTransformNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a SizePrefixTransformNode> for NodeRef<'a> {
    fn from(node: &'a SizePrefixTransformNode) -> Self {
        Self::SizePrefixTransform(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a SizePrefixTransformNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::SizePrefixTransform(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "sizePrefixTransformNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for SomeValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::SomeValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "someValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a SomeValueNode> for NodeRef<'a> {
    fn from(node: &'a SomeValueNode) -> Self {
        Self::SomeValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a SomeValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::SomeValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "someValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for StringDisplayNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::StringDisplay(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "stringDisplayNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a StringDisplayNode> for NodeRef<'a> {
    fn from(node: &'a StringDisplayNode) -> Self {
        Self::StringDisplay(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a StringDisplayNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::StringDisplay(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "stringDisplayNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for StringTypeNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::StringType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "stringTypeNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a StringTypeNode> for NodeRef<'a> {
    fn from(node: &'a StringTypeNode) -> Self {
        Self::StringType(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a StringTypeNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::StringType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "stringTypeNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for StringValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::StringValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "stringValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a StringValueNode> for NodeRef<'a> {
    fn from(node: &'a StringValueNode) -> Self {
        Self::StringValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a StringValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::StringValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "stringValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for StructFieldDisplayNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::StructFieldDisplay(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "structFieldDisplayNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a StructFieldDisplayNode> for NodeRef<'a> {
    fn from(node: &'a StructFieldDisplayNode) -> Self {
        Self::StructFieldDisplay(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a StructFieldDisplayNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::StructFieldDisplay(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "structFieldDisplayNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for StructFieldTypeNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::StructFieldType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "structFieldTypeNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a StructFieldTypeNode> for NodeRef<'a> {
    fn from(node: &'a StructFieldTypeNode) -> Self {
        Self::StructFieldType(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a StructFieldTypeNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::StructFieldType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "structFieldTypeNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for StructFieldValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::StructFieldValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "structFieldValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a StructFieldValueNode> for NodeRef<'a> {
    fn from(node: &'a StructFieldValueNode) -> Self {
        Self::StructFieldValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a StructFieldValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::StructFieldValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "structFieldValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for StructTypeNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::StructType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "structTypeNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a StructTypeNode> for NodeRef<'a> {
    fn from(node: &'a StructTypeNode) -> Self {
        Self::StructType(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a StructTypeNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::StructType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "structTypeNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for StructValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::StructValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "structValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a StructValueNode> for NodeRef<'a> {
    fn from(node: &'a StructValueNode) -> Self {
        Self::StructValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a StructValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::StructValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "structValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for TextNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Text(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "textNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a TextNode> for NodeRef<'a> {
    fn from(node: &'a TextNode) -> Self {
        Self::Text(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a TextNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::Text(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "textNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for TupleTypeNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::TupleType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "tupleTypeNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a TupleTypeNode> for NodeRef<'a> {
    fn from(node: &'a TupleTypeNode) -> Self {
        Self::TupleType(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a TupleTypeNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::TupleType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "tupleTypeNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for TupleValueNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::TupleValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "tupleValueNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a TupleValueNode> for NodeRef<'a> {
    fn from(node: &'a TupleValueNode) -> Self {
        Self::TupleValue(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a TupleValueNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::TupleValue(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "tupleValueNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for UnitNumberDisplayNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::UnitNumberDisplay(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "unitNumberDisplayNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a UnitNumberDisplayNode> for NodeRef<'a> {
    fn from(node: &'a UnitNumberDisplayNode) -> Self {
        Self::UnitNumberDisplay(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a UnitNumberDisplayNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::UnitNumberDisplay(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "unitNumberDisplayNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for ValueLabelDisplayNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::ValueLabelDisplay(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "valueLabelDisplayNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a ValueLabelDisplayNode> for NodeRef<'a> {
    fn from(node: &'a ValueLabelDisplayNode) -> Self {
        Self::ValueLabelDisplay(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a ValueLabelDisplayNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::ValueLabelDisplay(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "valueLabelDisplayNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for VariablePdaSeedNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::VariablePdaSeed(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "variablePdaSeedNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a VariablePdaSeedNode> for NodeRef<'a> {
    fn from(node: &'a VariablePdaSeedNode) -> Self {
        Self::VariablePdaSeed(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a VariablePdaSeedNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::VariablePdaSeed(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "variablePdaSeedNode".to_string(),
            }),
        }
    }
}

impl TryFrom<Node> for ZeroableOptionTypeNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::ZeroableOptionType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "zeroableOptionTypeNode".to_string(),
            }),
        }
    }
}

impl<'a> From<&'a ZeroableOptionTypeNode> for NodeRef<'a> {
    fn from(node: &'a ZeroableOptionTypeNode) -> Self {
        Self::ZeroableOptionType(node)
    }
}

impl<'a> TryFrom<NodeRef<'a>> for &'a ZeroableOptionTypeNode {
    type Error = CodamaError;

    fn try_from(node: NodeRef<'a>) -> Result<Self, Self::Error> {
        match node {
            NodeRef::ZeroableOptionType(node) => Ok(node),
            node => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "zeroableOptionTypeNode".to_string(),
            }),
        }
    }
}
