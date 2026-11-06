use codama_nodes_derive::node;
use serde_json::Value;

#[node]
#[derive(Default)]
pub struct PluginNode {
    // Data.
    pub namespace: String,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub payload: Option<Value>,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}
