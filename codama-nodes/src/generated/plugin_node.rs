use crate::CamelCaseString;
use codama_nodes_derive::node;
use serde_json::Value;

#[node]
#[derive(Default)]
pub struct PluginNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub payload: Option<Value>,
}
