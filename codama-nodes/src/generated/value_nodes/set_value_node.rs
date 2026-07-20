use crate::ValueNode;
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct SetValueNode {
    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub items: Vec<ValueNode>,
}

impl From<SetValueNode> for crate::Node {
    fn from(val: SetValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
