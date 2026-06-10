use crate::{CamelCaseString, HasName};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct ProgramLinkNode {
    // Data.
    pub name: CamelCaseString,
}

impl From<ProgramLinkNode> for crate::Node {
    fn from(val: ProgramLinkNode) -> Self {
        crate::Node::Link(val.into())
    }
}

impl HasName for ProgramLinkNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
