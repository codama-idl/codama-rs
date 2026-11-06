use crate::{CamelCaseString, HasIdentifier, Node};
use codama_nodes_derive::node;

#[node]
pub struct ProvidedNode {
    // Data.
    // TODO(spec v2): keeps the v1 JSON key "name" temporarily so v1 output stays the same. The next v2 regeneration drops it.
    #[serde(rename = "name")]
    pub identifier: CamelCaseString,

    // Children.
    pub node: Box<Node>,
}

impl HasIdentifier for ProvidedNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
