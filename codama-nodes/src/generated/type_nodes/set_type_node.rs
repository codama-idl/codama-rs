use crate::{CountNode, TypeNode};
use codama_nodes_derive::type_node;

#[type_node]
pub struct SetTypeNode {
    // Children.
    pub item: Box<TypeNode>,
    pub count: Box<CountNode>,
}

impl From<SetTypeNode> for crate::Node {
    fn from(val: SetTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
