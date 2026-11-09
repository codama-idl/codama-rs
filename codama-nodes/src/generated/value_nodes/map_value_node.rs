use crate::{MapEntryValueNode, PluginNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct MapValueNode {
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub entries: Vec<MapEntryValueNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}
