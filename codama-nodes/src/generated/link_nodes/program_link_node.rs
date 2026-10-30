use crate::{CamelCaseString, HasIdentifier};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct ProgramLinkNode {
    // Data.
    // TODO(spec v2): keeps the v1 JSON key "name" temporarily so v1 output stays the same. The next v2 regeneration drops it.
    #[serde(rename = "name")]
    pub identifier: CamelCaseString,
}

impl From<ProgramLinkNode> for crate::Node {
    fn from(val: ProgramLinkNode) -> Self {
        crate::Node::Link(val.into())
    }
}

impl HasIdentifier for ProgramLinkNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
