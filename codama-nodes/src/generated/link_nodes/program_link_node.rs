use crate::{HasIdentifier, Identifier, PluginNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct ProgramLinkNode {
    pub identifier: Identifier,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl HasIdentifier for ProgramLinkNode {
    fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}
