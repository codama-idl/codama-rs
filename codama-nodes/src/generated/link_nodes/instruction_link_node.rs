use crate::{CamelCaseString, HasName, ProgramLinkNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct InstructionLinkNode {
    // Data.
    pub name: CamelCaseString,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub program: Option<ProgramLinkNode>,
}

impl From<InstructionLinkNode> for crate::Node {
    fn from(val: InstructionLinkNode) -> Self {
        crate::Node::Link(val.into())
    }
}

impl HasName for InstructionLinkNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
