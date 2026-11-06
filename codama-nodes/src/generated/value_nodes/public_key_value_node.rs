use crate::{CamelCaseString, PluginNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct PublicKeyValueNode {
    // Data.
    pub public_key: String,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub identifier: Option<CamelCaseString>,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<PublicKeyValueNode> for crate::Node {
    fn from(val: PublicKeyValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
