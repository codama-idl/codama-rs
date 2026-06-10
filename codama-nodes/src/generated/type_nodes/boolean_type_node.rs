use crate::{NestedTypeNode, NumberTypeNode};
use codama_nodes_derive::type_node;

#[type_node]
pub struct BooleanTypeNode {
    // Children.
    pub size: NestedTypeNode<NumberTypeNode>,
}

impl From<BooleanTypeNode> for crate::Node {
    fn from(val: BooleanTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
