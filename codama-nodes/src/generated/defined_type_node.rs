use crate::{CamelCaseString, Docs, HasIdentifier, PluginNode, TypeNode};
use codama_nodes_derive::node;

#[node]
pub struct DefinedTypeNode {
    // Data.
    pub identifier: CamelCaseString,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,
    pub r#type: Box<TypeNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl HasIdentifier for DefinedTypeNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
