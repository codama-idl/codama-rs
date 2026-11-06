use crate::{CamelCaseString, HasIdentifier, ProgramLinkNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct AccountLinkNode {
    // Data.
    // TODO(spec v2): keeps the v1 JSON key "name" temporarily so v1 output stays the same. The next v2 regeneration drops it.
    #[serde(rename = "name")]
    pub identifier: CamelCaseString,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub program: Option<ProgramLinkNode>,
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
