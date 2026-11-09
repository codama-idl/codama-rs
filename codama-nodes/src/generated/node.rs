use crate::{
    AccountBumpValueNode, AccountDataValueNode, AccountLinkNode, AccountNode, AccountValueNode,
    AmountNumberDisplayNode, ArrayTypeNode, ArrayValueNode, BooleanTypeNode, BooleanValueNode,
    BytesTypeNode, BytesValueNode, ConditionalValueNode, ConstantDiscriminatorNode, ConstantNode,
    ConstantPdaSeedNode, ConstantValueNode, DataValueNode, DateTimeTypeNode, DefinedTypeLinkNode,
    DefinedTypeNode, DurationTypeNode, EnumTypeNode, EnumValueNode, EnumVariantDisplayNode,
    EnumVariantTypeNode, ErrorNode, EventNode, FieldDiscriminatorNode, FixedCountNode,
    FixedPointTypeNode, FixedSizeTransformNode, FloatTypeNode, FloatValueNode, HasKind,
    HiddenPrefixTransformNode, HiddenSuffixTransformNode, IdentityValueNode, InjectedValueNode,
    InstructionAccountDisplayNode, InstructionAccountLinkNode, InstructionAccountNode,
    InstructionByteDeltaNode, InstructionDisplayNode, InstructionLinkNode, InstructionNode,
    InstructionRemainingAccountsNode, InstructionStatusNode, IntegerTypeNode, IntegerValueNode,
    MapEntryValueNode, MapTypeNode, MapValueNode, NoneValueNode, OptionTypeNode, PayerValueNode,
    PdaLinkNode, PdaNode, PdaSeedValueNode, PdaValueNode, PluginNode, PostOffsetTransformNode,
    PreOffsetTransformNode, PrefixedCountNode, ProgramIdValueNode, ProgramLinkNode, ProgramNode,
    ProvidedNode, PublicKeyTypeNode, PublicKeyValueNode, RemainderCountNode,
    RemainderOptionTypeNode, RootNode, SentinelCountNode, SentinelTransformNode, SetTypeNode,
    SetValueNode, SizeDiscriminatorNode, SizePrefixTransformNode, SomeValueNode, StringDisplayNode,
    StringTypeNode, StringValueNode, StructFieldDisplayNode, StructFieldTypeNode,
    StructFieldValueNode, StructTypeNode, StructValueNode, TextNode, TupleTypeNode, TupleValueNode,
    UnitNumberDisplayNode, ValueLabelDisplayNode, VariablePdaSeedNode, ZeroableOptionTypeNode,
};
use codama_errors::CodamaError;
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

impl NodeRef<'_> {
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
