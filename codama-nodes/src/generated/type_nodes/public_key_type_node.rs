use crate::{AccountLinkNode, PluginNode, TransformNode};
use codama_nodes_derive::type_node;

#[type_node]
#[derive(Default)]
pub struct PublicKeyTypeNode {
    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub account_link: Option<AccountLinkNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub transforms: Vec<TransformNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<PublicKeyTypeNode> for crate::Node {
    fn from(val: PublicKeyTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
