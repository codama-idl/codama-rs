use crate::{Identifier, PluginNode, ValueNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct InjectedValueNode {
    pub key: Identifier,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub fallback: Box<Option<ValueNode>>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}
