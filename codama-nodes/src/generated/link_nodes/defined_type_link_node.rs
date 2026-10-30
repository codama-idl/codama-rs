use crate::{CamelCaseString, HasIdentifier, ProgramLinkNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct DefinedTypeLinkNode {
    // Data.
    // TODO(spec v2): keeps the v1 JSON key "name" temporarily so v1 output stays the same. The next v2 regeneration drops it.
    #[serde(rename = "name")]
    pub identifier: CamelCaseString,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub program: Option<ProgramLinkNode>,
}

impl From<DefinedTypeLinkNode> for crate::Node {
    fn from(val: DefinedTypeLinkNode) -> Self {
        crate::Node::Link(val.into())
    }
}

impl HasIdentifier for DefinedTypeLinkNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
