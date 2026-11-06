use crate::{CamelCaseString, HasIdentifier, PluginNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct ProgramLinkNode {
    // Data.
    pub identifier: CamelCaseString,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
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
