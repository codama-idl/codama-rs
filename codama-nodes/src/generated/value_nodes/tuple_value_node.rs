use crate::ValueNode;
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct TupleValueNode {
    // Children.
    pub items: Vec<ValueNode>,
}

impl From<TupleValueNode> for crate::Node {
    fn from(val: TupleValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
