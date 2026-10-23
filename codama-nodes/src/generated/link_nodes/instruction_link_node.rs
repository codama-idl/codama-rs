use crate::{CamelCaseString, HasIdentifier, PluginNode, ProgramLinkNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct InstructionLinkNode {
    // Data.
    pub identifier: CamelCaseString,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub program: Option<ProgramLinkNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<InstructionLinkNode> for crate::Node {
    fn from(val: InstructionLinkNode) -> Self {
        crate::Node::Link(val.into())
    }
}

impl HasIdentifier for InstructionLinkNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
