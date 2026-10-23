use crate::{PluginNode, ValueNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct SetValueNode {
    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub items: Vec<ValueNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<SetValueNode> for crate::Node {
    fn from(val: SetValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
