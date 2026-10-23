use crate::{CamelCaseString, HasIdentifier, PluginNode, ProgramLinkNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct PdaLinkNode {
    // Data.
    pub identifier: CamelCaseString,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub program: Option<ProgramLinkNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<PdaLinkNode> for crate::Node {
    fn from(val: PdaLinkNode) -> Self {
        crate::Node::Link(val.into())
    }
}

impl HasIdentifier for PdaLinkNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
