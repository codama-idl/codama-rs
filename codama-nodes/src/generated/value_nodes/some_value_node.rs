use crate::{PluginNode, ValueNode};
use codama_nodes_derive::node;

#[node]
pub struct SomeValueNode {
    // Children.
    pub value: Box<ValueNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<SomeValueNode> for crate::Node {
    fn from(val: SomeValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
