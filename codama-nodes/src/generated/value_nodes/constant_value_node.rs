use crate::{TypeNode, ValueNode};
use codama_nodes_derive::node;

#[node]
pub struct ConstantValueNode {
    // Children.
    pub r#type: Box<TypeNode>,
    pub value: Box<ValueNode>,
}

impl From<ConstantValueNode> for crate::Node {
    fn from(val: ConstantValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
