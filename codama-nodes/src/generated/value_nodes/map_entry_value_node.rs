use crate::{PluginNode, ValueNode};
use codama_nodes_derive::node;

#[node]
pub struct MapEntryValueNode {
    // Children.
    pub key: Box<ValueNode>,
    pub value: Box<ValueNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<MapEntryValueNode> for crate::Node {
    fn from(val: MapEntryValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
