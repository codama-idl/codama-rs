use crate::{Docs, HasIdentifier, Identifier, PluginNode, Text};
use codama_nodes_derive::node;

#[node]
pub struct ErrorNode {
    pub identifier: Identifier,
    pub code: u32,
    pub message: Text,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl HasIdentifier for ErrorNode {
    fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}
