use crate::{
    AmountTypeNode, ArrayTypeNode, BooleanTypeNode, BytesTypeNode, DateTimeTypeNode,
    EnumEmptyVariantTypeNode, EnumStructVariantTypeNode, EnumTupleVariantTypeNode, EnumTypeNode,
    FixedSizeTypeNode, HiddenPrefixTypeNode, HiddenSuffixTypeNode, MapTypeNode, NumberTypeNode,
    OptionTypeNode, PostOffsetTypeNode, PreOffsetTypeNode, PublicKeyTypeNode,
    RemainderOptionTypeNode, SentinelTypeNode, SetTypeNode, SizePrefixTypeNode, SolAmountTypeNode,
    StringTypeNode, StructFieldTypeNode, StructTypeNode, TupleTypeNode, TypeNodeEnumTrait,
    ZeroableOptionTypeNode,
};
use codama_nodes_derive::{node_union, RegisteredNodes};

#[derive(RegisteredNodes)]
#[node_union]
pub enum RegisteredTypeNode {
    Amount(AmountTypeNode),
    Array(Box<ArrayTypeNode>),
    Boolean(BooleanTypeNode),
    Bytes(BytesTypeNode),
    DateTime(DateTimeTypeNode),
    Enum(EnumTypeNode),
    FixedSize(Box<FixedSizeTypeNode<TypeNode>>),
    HiddenPrefix(Box<HiddenPrefixTypeNode<TypeNode>>),
    HiddenSuffix(Box<HiddenSuffixTypeNode<TypeNode>>),
    Map(Box<MapTypeNode>),
    Number(NumberTypeNode),
    Option(Box<OptionTypeNode>),
    PostOffset(Box<PostOffsetTypeNode<TypeNode>>),
    PreOffset(Box<PreOffsetTypeNode<TypeNode>>),
    PublicKey(PublicKeyTypeNode),
    RemainderOption(Box<RemainderOptionTypeNode>),
    Sentinel(Box<SentinelTypeNode<TypeNode>>),
    Set(Box<SetTypeNode>),
    SizePrefix(Box<SizePrefixTypeNode<TypeNode>>),
    SolAmount(SolAmountTypeNode),
    String(StringTypeNode),
    Struct(StructTypeNode),
    Tuple(TupleTypeNode),
    ZeroableOption(Box<ZeroableOptionTypeNode>),

    #[registered]
    EnumEmptyVariant(EnumEmptyVariantTypeNode),
    #[registered]
    EnumStructVariant(EnumStructVariantTypeNode),
    #[registered]
    EnumTupleVariant(EnumTupleVariantTypeNode),
    #[registered]
    StructField(StructFieldTypeNode),
}

impl TypeNodeEnumTrait for TypeNode {}
