use crate::{CountNode, TypeNode};
use codama_nodes_derive::type_node;

#[type_node]
pub struct MapTypeNode {
    // Children.
    pub key: Box<TypeNode>,
    pub value: Box<TypeNode>,
    pub count: Box<CountNode>,
}

impl From<MapTypeNode> for crate::Node {
    fn from(val: MapTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
