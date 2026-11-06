use crate::{CamelCaseString, HasIdentifier, PluginNode, ProgramLinkNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct AccountLinkNode {
    // Data.
    pub identifier: CamelCaseString,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub program: Option<ProgramLinkNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<AccountLinkNode> for crate::Node {
    fn from(val: AccountLinkNode) -> Self {
        crate::Node::Link(val.into())
    }
}

impl HasIdentifier for AccountLinkNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
