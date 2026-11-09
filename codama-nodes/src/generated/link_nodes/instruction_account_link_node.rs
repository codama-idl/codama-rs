use crate::{HasIdentifier, Identifier, InstructionLinkNode, PluginNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct InstructionAccountLinkNode {
    pub identifier: Identifier,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub instruction: Option<InstructionLinkNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl HasIdentifier for InstructionAccountLinkNode {
    fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}
