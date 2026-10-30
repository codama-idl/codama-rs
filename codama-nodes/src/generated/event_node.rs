use crate::{CamelCaseString, DiscriminatorNode, Docs, HasIdentifier, TypeNode};
use codama_nodes_derive::node;

#[node]
pub struct EventNode {
    // Data.
    // TODO(spec v2): keeps the v1 JSON key "name" temporarily so v1 output stays the same. The next v2 regeneration drops it.
    #[serde(rename = "name")]
    pub identifier: CamelCaseString,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,

    // Children.
    pub data: Box<TypeNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub discriminators: Vec<DiscriminatorNode>,
}

impl HasIdentifier for EventNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
