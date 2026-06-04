use crate::ValueNode;
use codama_nodes_derive::node;

#[node]
pub struct SomeValueNode {
    // Children.
    pub value: Box<ValueNode>,
}

impl From<SomeValueNode> for crate::Node {
    fn from(val: SomeValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
