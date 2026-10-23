use crate::{MapEntryValueNode, PluginNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct MapValueNode {
    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub entries: Vec<MapEntryValueNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<MapValueNode> for crate::Node {
    fn from(val: MapValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
