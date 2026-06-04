use crate::StructFieldValueNode;
use codama_nodes_derive::node;

#[node]
pub struct StructValueNode {
    // Children.
    pub fields: Vec<StructFieldValueNode>,
}

impl From<StructValueNode> for crate::Node {
    fn from(val: StructValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
