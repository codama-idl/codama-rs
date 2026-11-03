use crate::Namespace;
use codama_nodes_derive::node;
use serde_json::Value;

#[node]
pub struct PluginNode {
    pub namespace: Namespace,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub payload: Option<Value>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}
