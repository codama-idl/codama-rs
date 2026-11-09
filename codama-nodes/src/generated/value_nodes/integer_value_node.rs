use crate::{Integer, PluginNode};
use codama_nodes_derive::node;

#[node]
pub struct IntegerValueNode {
    pub value: Integer,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}
