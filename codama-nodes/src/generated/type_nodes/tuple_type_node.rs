use crate::TypeNode;
use codama_nodes_derive::type_node;

#[type_node]
#[derive(Default)]
pub struct TupleTypeNode {
    // Children.
    pub items: Vec<TypeNode>,
}

impl From<TupleTypeNode> for crate::Node {
    fn from(val: TupleTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
