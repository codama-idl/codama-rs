use crate::{CamelCaseString, HasName, InstructionLinkNode};
use codama_nodes_derive::node;

#[node]
pub struct InstructionArgumentLinkNode {
    // Data.
    pub name: CamelCaseString,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub instruction: Option<InstructionLinkNode>,
}

impl From<InstructionArgumentLinkNode> for crate::Node {
    fn from(val: InstructionArgumentLinkNode) -> Self {
        crate::Node::Link(val.into())
    }
}

impl HasName for InstructionArgumentLinkNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
