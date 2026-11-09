use crate::PluginNode;
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct StringDisplayNode {
    #[serde(skip_serializing_if = "crate::is_default")]
    pub slice_start: Option<u64>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub slice_end: Option<u64>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}
