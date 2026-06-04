use crate::{CamelCaseString, HasName, ProgramLinkNode};
use codama_nodes_derive::node;

#[node]
pub struct AccountLinkNode {
    // Data.
    pub name: CamelCaseString,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub program: Option<ProgramLinkNode>,
}

impl From<AccountLinkNode> for crate::Node {
    fn from(val: AccountLinkNode) -> Self {
        crate::Node::Link(val.into())
    }
}

impl HasName for AccountLinkNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
