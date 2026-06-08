use crate::{CamelCaseString, DiscriminatorNode, Docs, HasName, TypeNode};
use codama_nodes_derive::node;

#[node]
pub struct EventNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,

    // Children.
    pub data: Box<TypeNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub discriminators: Vec<DiscriminatorNode>,
}

impl HasName for EventNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
