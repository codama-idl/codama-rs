use crate::{
    AmountTypeNode, ArrayTypeNode, BooleanTypeNode, BytesTypeNode, DateTimeTypeNode,
    DefinedTypeLinkNode, EnumEmptyVariantTypeNode, EnumStructVariantTypeNode,
    EnumTupleVariantTypeNode, EnumTypeNode, FixedSizeTypeNode, HiddenPrefixTypeNode,
    HiddenSuffixTypeNode, LinkNode, MapTypeNode, Node, NodeTrait, NodeUnionTrait, NumberTypeNode,
    OptionTypeNode, PostOffsetTypeNode, PreOffsetTypeNode, PublicKeyTypeNode,
    RemainderOptionTypeNode, SentinelTypeNode, SetTypeNode, SizePrefixTypeNode, SolAmountTypeNode,
    StringTypeNode, StructFieldTypeNode, StructTypeNode, TupleTypeNode, TypeNodeUnionTrait,
    ZeroableOptionTypeNode,
};
use codama_errors::CodamaError;
use codama_nodes_derive::node_union;

#[node_union]
pub enum RegisteredTypeNode {
    Amount(AmountTypeNode),
    Array(ArrayTypeNode),
    Boolean(BooleanTypeNode),
    Bytes(BytesTypeNode),
    DateTime(DateTimeTypeNode),
    Enum(EnumTypeNode),
    FixedSize(FixedSizeTypeNode<TypeNode>),
    HiddenPrefix(HiddenPrefixTypeNode<TypeNode>),
    HiddenSuffix(HiddenSuffixTypeNode<TypeNode>),
    Map(MapTypeNode),
    Number(NumberTypeNode),
    Option(OptionTypeNode),
    PostOffset(PostOffsetTypeNode<TypeNode>),
    PreOffset(PreOffsetTypeNode<TypeNode>),
    PublicKey(PublicKeyTypeNode),
    RemainderOption(RemainderOptionTypeNode),
    Sentinel(SentinelTypeNode<TypeNode>),
    Set(SetTypeNode),
    SizePrefix(SizePrefixTypeNode<TypeNode>),
    SolAmount(SolAmountTypeNode),
    String(StringTypeNode),
    Struct(StructTypeNode),
    Tuple(TupleTypeNode),
    ZeroableOption(ZeroableOptionTypeNode),

    // Registered only.
    EnumEmptyVariant(EnumEmptyVariantTypeNode),
    EnumStructVariant(EnumStructVariantTypeNode),
    EnumTupleVariant(EnumTupleVariantTypeNode),
    StructField(StructFieldTypeNode),
}

#[node_union]
pub enum TypeNode {
    Amount(AmountTypeNode),
    Array(ArrayTypeNode),
    Boolean(BooleanTypeNode),
    Bytes(BytesTypeNode),
    DateTime(DateTimeTypeNode),
    Enum(EnumTypeNode),
    FixedSize(FixedSizeTypeNode<TypeNode>),
    HiddenPrefix(HiddenPrefixTypeNode<TypeNode>),
    HiddenSuffix(HiddenSuffixTypeNode<TypeNode>),
    Map(MapTypeNode),
    Number(NumberTypeNode),
    Option(OptionTypeNode),
    PostOffset(PostOffsetTypeNode<TypeNode>),
    PreOffset(PreOffsetTypeNode<TypeNode>),
    PublicKey(PublicKeyTypeNode),
    RemainderOption(RemainderOptionTypeNode),
    Sentinel(SentinelTypeNode<TypeNode>),
    Set(SetTypeNode),
    SizePrefix(SizePrefixTypeNode<TypeNode>),
    SolAmount(SolAmountTypeNode),
    String(StringTypeNode),
    Struct(StructTypeNode),
    Tuple(TupleTypeNode),
    ZeroableOption(ZeroableOptionTypeNode),

    // Standalone only.
    Link(DefinedTypeLinkNode),
}

impl TypeNodeUnionTrait for TypeNode {}

impl TryFrom<Node> for TypeNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Type(node) => Self::try_from(node),
            Node::Link(LinkNode::DefinedType(node)) => Ok(Self::Link(node)),
            _ => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "TypeNode".to_string(),
            }),
        }
    }
}

impl<T> TryFrom<Option<T>> for TypeNode
where
    T: TryInto<Self, Error = CodamaError>,
{
    type Error = CodamaError;

    fn try_from(node: Option<T>) -> Result<Self, Self::Error> {
        match node {
            Some(t) => t.try_into(),
            _ => Err(CodamaError::InvalidNodeConversion {
                from: "None".to_string(),
                into: "TypeNode".to_string(),
            }),
        }
    }
}

impl TryFrom<RegisteredTypeNode> for TypeNode {
    type Error = CodamaError;

    fn try_from(node: RegisteredTypeNode) -> Result<Self, Self::Error> {
        match node {
            RegisteredTypeNode::Amount(node) => Ok(Self::Amount(node)),
            RegisteredTypeNode::Array(node) => Ok(Self::Array(node)),
            RegisteredTypeNode::Boolean(node) => Ok(Self::Boolean(node)),
            RegisteredTypeNode::Bytes(node) => Ok(Self::Bytes(node)),
            RegisteredTypeNode::DateTime(node) => Ok(Self::DateTime(node)),
            RegisteredTypeNode::Enum(node) => Ok(Self::Enum(node)),
            RegisteredTypeNode::FixedSize(node) => Ok(Self::FixedSize(node)),
            RegisteredTypeNode::HiddenPrefix(node) => Ok(Self::HiddenPrefix(node)),
            RegisteredTypeNode::HiddenSuffix(node) => Ok(Self::HiddenSuffix(node)),
            RegisteredTypeNode::Map(node) => Ok(Self::Map(node)),
            RegisteredTypeNode::Number(node) => Ok(Self::Number(node)),
            RegisteredTypeNode::Option(node) => Ok(Self::Option(node)),
            RegisteredTypeNode::PostOffset(node) => Ok(Self::PostOffset(node)),
            RegisteredTypeNode::PreOffset(node) => Ok(Self::PreOffset(node)),
            RegisteredTypeNode::PublicKey(node) => Ok(Self::PublicKey(node)),
            RegisteredTypeNode::RemainderOption(node) => Ok(Self::RemainderOption(node)),
            RegisteredTypeNode::Sentinel(node) => Ok(Self::Sentinel(node)),
            RegisteredTypeNode::Set(node) => Ok(Self::Set(node)),
            RegisteredTypeNode::SizePrefix(node) => Ok(Self::SizePrefix(node)),
            RegisteredTypeNode::SolAmount(node) => Ok(Self::SolAmount(node)),
            RegisteredTypeNode::String(node) => Ok(Self::String(node)),
            RegisteredTypeNode::Struct(node) => Ok(Self::Struct(node)),
            RegisteredTypeNode::Tuple(node) => Ok(Self::Tuple(node)),
            RegisteredTypeNode::ZeroableOption(node) => Ok(Self::ZeroableOption(node)),
            _ => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "TypeNode".to_string(),
            }),
        }
    }
}

impl TryFrom<TypeNode> for RegisteredTypeNode {
    type Error = CodamaError;

    fn try_from(node: TypeNode) -> Result<Self, Self::Error> {
        match node {
            TypeNode::Amount(node) => Ok(Self::Amount(node)),
            TypeNode::Array(node) => Ok(Self::Array(node)),
            TypeNode::Boolean(node) => Ok(Self::Boolean(node)),
            TypeNode::Bytes(node) => Ok(Self::Bytes(node)),
            TypeNode::DateTime(node) => Ok(Self::DateTime(node)),
            TypeNode::Enum(node) => Ok(Self::Enum(node)),
            TypeNode::FixedSize(node) => Ok(Self::FixedSize(node)),
            TypeNode::HiddenPrefix(node) => Ok(Self::HiddenPrefix(node)),
            TypeNode::HiddenSuffix(node) => Ok(Self::HiddenSuffix(node)),
            TypeNode::Map(node) => Ok(Self::Map(node)),
            TypeNode::Number(node) => Ok(Self::Number(node)),
            TypeNode::Option(node) => Ok(Self::Option(node)),
            TypeNode::PostOffset(node) => Ok(Self::PostOffset(node)),
            TypeNode::PreOffset(node) => Ok(Self::PreOffset(node)),
            TypeNode::PublicKey(node) => Ok(Self::PublicKey(node)),
            TypeNode::RemainderOption(node) => Ok(Self::RemainderOption(node)),
            TypeNode::Sentinel(node) => Ok(Self::Sentinel(node)),
            TypeNode::Set(node) => Ok(Self::Set(node)),
            TypeNode::SizePrefix(node) => Ok(Self::SizePrefix(node)),
            TypeNode::SolAmount(node) => Ok(Self::SolAmount(node)),
            TypeNode::String(node) => Ok(Self::String(node)),
            TypeNode::Struct(node) => Ok(Self::Struct(node)),
            TypeNode::Tuple(node) => Ok(Self::Tuple(node)),
            TypeNode::ZeroableOption(node) => Ok(Self::ZeroableOption(node)),
            _ => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "RegisteredTypeNode".to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NodeUnionTrait;

    #[test]
    fn kind_from_standalone() {
        let node: TypeNode = StringTypeNode::utf8().into();
        assert_eq!(node.kind(), "stringTypeNode");
    }

    #[test]
    fn kind_from_registered() {
        let node: RegisteredTypeNode = StringTypeNode::utf8().into();
        assert_eq!(node.kind(), "stringTypeNode");
    }

    #[test]
    fn try_from_node_ok() {
        let node: Node = StringTypeNode::utf8().into();
        let result = TypeNode::try_from(node);
        assert!(matches!(result, Ok(TypeNode::String(_))));
    }

    #[test]
    fn try_from_node_err() {
        let node: Node = StructFieldTypeNode::new("foo", StringTypeNode::utf8()).into();
        let result = TypeNode::try_from(node);
        assert!(matches!(
            result,
            Err(CodamaError::InvalidNodeConversion { from, into }) if from == "structFieldTypeNode" && into == "TypeNode"
        ));
    }
}
