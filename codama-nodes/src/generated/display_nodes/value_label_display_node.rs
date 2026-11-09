use crate::{PluginNode, Text, ValueNode};
use codama_nodes_derive::node;

#[node]
pub struct ValueLabelDisplayNode {
    pub label: Text,
    pub value: Box<ValueNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}
