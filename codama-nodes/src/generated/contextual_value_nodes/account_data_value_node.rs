use crate::{CamelCaseString, PluginNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct AccountDataValueNode {
    // Data.
    pub account: CamelCaseString,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub path: Option<String>,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<AccountDataValueNode> for crate::Node {
    fn from(val: AccountDataValueNode) -> Self {
        crate::Node::ContextualValue(val.into())
    }
}
