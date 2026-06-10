use crate::TypeNode;
use codama_nodes_derive::type_node;

#[type_node]
pub struct RemainderOptionTypeNode {
    // Children.
    pub item: Box<TypeNode>,
}

impl From<RemainderOptionTypeNode> for crate::Node {
    fn from(val: RemainderOptionTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
