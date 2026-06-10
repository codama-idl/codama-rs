use crate::{CamelCaseString, HasName, ProgramLinkNode};
use codama_nodes_derive::node;

#[node]
pub struct PdaLinkNode {
    // Data.
    pub name: CamelCaseString,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub program: Option<ProgramLinkNode>,
}

impl From<PdaLinkNode> for crate::Node {
    fn from(val: PdaLinkNode) -> Self {
        crate::Node::Link(val.into())
    }
}

impl HasName for PdaLinkNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
