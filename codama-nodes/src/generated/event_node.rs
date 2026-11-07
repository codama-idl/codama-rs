use crate::{DiscriminatorNode, Docs, HasIdentifier, Identifier, PluginNode, TypeNode};
use codama_nodes_derive::node;

#[node]
pub struct EventNode {
    pub identifier: Identifier,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,
    pub data: Box<TypeNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub discriminators: Vec<DiscriminatorNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl HasIdentifier for EventNode {
    fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}
