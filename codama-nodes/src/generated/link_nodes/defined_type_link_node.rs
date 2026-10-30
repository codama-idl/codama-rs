use crate::{CamelCaseString, HasIdentifier, PluginNode, ProgramLinkNode, TransformNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct DefinedTypeLinkNode {
    // Data.
    pub identifier: CamelCaseString,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub program: Option<ProgramLinkNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub transforms: Vec<TransformNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
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
