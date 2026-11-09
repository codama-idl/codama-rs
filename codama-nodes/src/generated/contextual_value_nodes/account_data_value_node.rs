use crate::{Identifier, Path, PluginNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct AccountDataValueNode {
    pub account: Identifier,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub path: Option<Path>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}
