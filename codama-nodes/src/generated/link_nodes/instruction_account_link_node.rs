use crate::{CamelCaseString, HasName, InstructionLinkNode};
use codama_nodes_derive::node;

#[node]
pub struct InstructionAccountLinkNode {
    // Data.
    pub name: CamelCaseString,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub instruction: Option<InstructionLinkNode>,
}

impl From<InstructionAccountLinkNode> for crate::Node {
    fn from(val: InstructionAccountLinkNode) -> Self {
        crate::Node::Link(val.into())
    }
}

impl HasName for InstructionAccountLinkNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
