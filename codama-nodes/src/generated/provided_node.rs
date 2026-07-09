use crate::{CamelCaseString, HasName, Node};
use codama_nodes_derive::node;

#[node]
pub struct ProvidedNode {
    // Data.
    pub name: CamelCaseString,

    // Children.
    pub node: Box<Node>,
}

impl HasName for ProvidedNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
