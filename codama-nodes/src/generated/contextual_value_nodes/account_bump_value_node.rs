use crate::{HasIdentifier, Identifier, PluginNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct AccountBumpValueNode {
    pub identifier: Identifier,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<AccountBumpValueNode> for crate::Node {
    fn from(val: AccountBumpValueNode) -> Self {
        crate::Node::ContextualValue(val.into())
    }
}

impl HasIdentifier for AccountBumpValueNode {
    fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}
