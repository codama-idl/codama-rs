use crate::{CountNode, TypeNode};
use codama_nodes_derive::type_node;

#[type_node]
pub struct ArrayTypeNode {
    // Children.
    pub item: Box<TypeNode>,
    pub count: Box<CountNode>,
}

impl From<ArrayTypeNode> for crate::Node {
    fn from(val: ArrayTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
