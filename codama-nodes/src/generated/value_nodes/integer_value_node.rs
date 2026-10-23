use crate::{Integer, PluginNode};
use codama_nodes_derive::node;

#[node]
pub struct IntegerValueNode {
    // Data.
    pub value: Integer,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<IntegerValueNode> for crate::Node {
    fn from(val: IntegerValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
