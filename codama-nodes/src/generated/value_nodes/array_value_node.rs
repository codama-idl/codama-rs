use crate::ValueNode;
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct ArrayValueNode {
    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub items: Vec<ValueNode>,
}

impl From<ArrayValueNode> for crate::Node {
    fn from(val: ArrayValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
