use crate::{NestedTypeNode, NumberTypeNode};
use codama_nodes_derive::type_node;

#[type_node]
pub struct SolAmountTypeNode {
    // Children.
    pub number: NestedTypeNode<NumberTypeNode>,
}

impl From<SolAmountTypeNode> for crate::Node {
    fn from(val: SolAmountTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
