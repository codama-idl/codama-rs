use crate::{CamelCaseString, HasIdentifier, InstructionLinkNode, PluginNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct InstructionAccountLinkNode {
    // Data.
    pub identifier: CamelCaseString,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub instruction: Option<InstructionLinkNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<InstructionAccountLinkNode> for crate::Node {
    fn from(val: InstructionAccountLinkNode) -> Self {
        crate::Node::Link(val.into())
    }
}

impl HasIdentifier for InstructionAccountLinkNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
