use crate::{
    EnumEmptyVariantTypeNode, EnumStructVariantTypeNode, EnumTupleVariantTypeNode, HasKind, Node,
    RegisteredTypeNode,
};
use codama_errors::CodamaError;
use codama_nodes_derive::node_union;

#[node_union]
pub enum EnumVariantTypeNode {
    Empty(EnumEmptyVariantTypeNode),
    Struct(EnumStructVariantTypeNode),
    Tuple(EnumTupleVariantTypeNode),
}

impl TryFrom<Node> for EnumVariantTypeNode {
    type Error = CodamaError;

    fn try_from(value: Node) -> Result<Self, Self::Error> {
        match value {
            Node::Type(RegisteredTypeNode::EnumEmptyVariant(node)) => {
                Ok(EnumVariantTypeNode::Empty(node.clone()))
            }
            Node::Type(RegisteredTypeNode::EnumTupleVariant(node)) => {
                Ok(EnumVariantTypeNode::Tuple(node.clone()))
            }
            Node::Type(RegisteredTypeNode::EnumStructVariant(node)) => {
                Ok(EnumVariantTypeNode::Struct(node.clone()))
            }
            _ => Err(CodamaError::InvalidNodeConversion {
                from: value.kind().into(),
                into: "EnumVariantTypeNode".into(),
            }),
        }
    }
}
