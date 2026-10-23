use crate::{CamelCaseString, HasIdentifier};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct AccountBumpValueNode {
    // Data.
    // TODO(spec v2): keeps the v1 JSON key "name" temporarily so v1 output stays the same. The next v2 regeneration drops it.
    #[serde(rename = "name")]
    pub identifier: CamelCaseString,
}

impl From<AccountBumpValueNode> for crate::Node {
    fn from(val: AccountBumpValueNode) -> Self {
        crate::Node::ContextualValue(val.into())
    }
}

impl HasIdentifier for AccountBumpValueNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
