use crate::{CamelCaseString, HasIdentifier, InstructionLinkNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct InstructionAccountLinkNode {
    // Data.
    // TODO(spec v2): keeps the v1 JSON key "name" temporarily so v1 output stays the same. The next v2 regeneration drops it.
    #[serde(rename = "name")]
    pub identifier: CamelCaseString,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub instruction: Option<InstructionLinkNode>,
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
