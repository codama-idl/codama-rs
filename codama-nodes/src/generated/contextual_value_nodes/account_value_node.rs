use crate::{CamelCaseString, HasName};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct AccountValueNode {
    // Data.
    pub name: CamelCaseString,
}

impl From<AccountValueNode> for crate::Node {
    fn from(val: AccountValueNode) -> Self {
        crate::Node::ContextualValue(val.into())
    }
}

impl HasName for AccountValueNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
