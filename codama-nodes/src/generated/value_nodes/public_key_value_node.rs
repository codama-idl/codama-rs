use crate::{Identifier, PluginNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct PublicKeyValueNode {
    pub public_key: String,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub identifier: Option<Identifier>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}
