use crate::{NestedTypeNode, NumberTypeNode};
use codama_nodes_derive::type_node;

#[type_node]
pub struct DateTimeTypeNode {
    // Children.
    pub number: NestedTypeNode<NumberTypeNode>,
}

impl From<DateTimeTypeNode> for crate::Node {
    fn from(val: DateTimeTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
