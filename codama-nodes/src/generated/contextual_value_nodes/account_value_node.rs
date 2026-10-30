use crate::{CamelCaseString, HasIdentifier, PluginNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct AccountValueNode {
    // Data.
    pub identifier: CamelCaseString,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<AccountValueNode> for crate::Node {
    fn from(val: AccountValueNode) -> Self {
        crate::Node::ContextualValue(val.into())
    }
}

impl HasIdentifier for AccountValueNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
