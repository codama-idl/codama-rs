use crate::{HasIdentifier, Identifier, Node, PluginNode};
use codama_nodes_derive::node;

#[node]
pub struct ProvidedNode {
    pub identifier: Identifier,
    pub node: Box<Node>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl HasIdentifier for ProvidedNode {
    fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}
