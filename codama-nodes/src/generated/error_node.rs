use crate::{CamelCaseString, Docs, HasIdentifier, PluginNode, Text};
use codama_nodes_derive::node;

#[node]
pub struct ErrorNode {
    // Data.
    pub identifier: CamelCaseString,
    pub code: u32,

    // Children.
    pub message: Text,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl HasIdentifier for ErrorNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
