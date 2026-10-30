use crate::{CamelCaseString, HasIdentifier, PluginNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct AccountBumpValueNode {
    // Data.
    pub identifier: CamelCaseString,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
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
