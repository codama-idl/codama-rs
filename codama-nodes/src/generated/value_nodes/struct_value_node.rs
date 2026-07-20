use crate::StructFieldValueNode;
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct StructValueNode {
    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub fields: Vec<StructFieldValueNode>,
}

impl From<StructValueNode> for crate::Node {
    fn from(val: StructValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
