use crate::{CamelCaseString, HasName};
use codama_nodes_derive::node;

#[node]
pub struct FieldDiscriminatorNode {
    // Data.
    pub name: CamelCaseString,
    pub offset: u64,
}

impl From<FieldDiscriminatorNode> for crate::Node {
    fn from(val: FieldDiscriminatorNode) -> Self {
        crate::Node::Discriminator(val.into())
    }
}

impl HasName for FieldDiscriminatorNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
