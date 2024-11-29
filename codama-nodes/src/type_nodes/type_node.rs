use crate::{
    AmountTypeNode, ArrayTypeNode, BooleanTypeNode, BytesTypeNode, DateTimeTypeNode,
    EnumEmptyVariantTypeNode, EnumStructVariantTypeNode, EnumTupleVariantTypeNode, EnumTypeNode,
    FixedSizeTypeNode, HiddenPrefixTypeNode, HiddenSuffixTypeNode, MapTypeNode, Node,
    NumberTypeNode, OptionTypeNode, PostOffsetTypeNode, PreOffsetTypeNode, PublicKeyTypeNode,
    RemainderOptionTypeNode, SentinelTypeNode, SetTypeNode, SizePrefixTypeNode, SolAmountTypeNode,
    StringTypeNode, StructFieldTypeNode, StructTypeNode, TupleTypeNode, TypeNodeUnionTrait,
    ZeroableOptionTypeNode,
};
use codama_errors::CodamaError;
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

impl TypeNodeUnionTrait for TypeNode {}

impl TryFrom<Node> for TypeNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Type(node) => Self::try_from(node),
            _ => Err(CodamaError::InvalidNodeConversion {
                from: "node.kind().to_string()".to_string(), // TODO
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
}
