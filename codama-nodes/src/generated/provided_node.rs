use crate::{CamelCaseString, HasIdentifier, Node, PluginNode};
use codama_nodes_derive::node;

#[node]
pub struct ProvidedNode {
    // Data.
    pub identifier: CamelCaseString,

    // Children.
    pub node: Box<Node>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl HasIdentifier for ProvidedNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
