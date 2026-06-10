use crate::Number;
use codama_nodes_derive::node;

#[node]
#[derive(Copy)]
pub struct NumberValueNode {
    // Data.
    pub number: Number,
}

impl From<NumberValueNode> for crate::Node {
    fn from(val: NumberValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
