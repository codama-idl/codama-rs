use crate::{CamelCaseString, HasName};
use codama_nodes_derive::node;

#[node]
pub struct AccountBumpValueNode {
    // Data.
    pub name: CamelCaseString,
}

impl From<AccountBumpValueNode> for crate::Node {
    fn from(val: AccountBumpValueNode) -> Self {
        crate::Node::ContextualValue(val.into())
    }
}

impl HasName for AccountBumpValueNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
