use crate::{Path, PluginNode};
use codama_nodes_derive::node;

#[node]
pub struct FieldDiscriminatorNode {
    pub path: Path,
    pub offset: u64,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}
