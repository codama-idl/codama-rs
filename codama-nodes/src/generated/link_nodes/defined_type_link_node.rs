use crate::{CamelCaseString, HasName, ProgramLinkNode};
use codama_nodes_derive::node;

#[node]
pub struct DefinedTypeLinkNode {
    // Data.
    pub name: CamelCaseString,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub program: Option<ProgramLinkNode>,
}

impl From<DefinedTypeLinkNode> for crate::Node {
    fn from(val: DefinedTypeLinkNode) -> Self {
        crate::Node::Link(val.into())
    }
}

impl HasName for DefinedTypeLinkNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
