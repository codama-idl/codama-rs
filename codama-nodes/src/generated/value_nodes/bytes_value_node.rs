use crate::{BytesEncoding, PluginNode};
use codama_nodes_derive::node;

#[node]
pub struct BytesValueNode {
    // Data.
    pub data: String,
    pub encoding: BytesEncoding,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<BytesValueNode> for crate::Node {
    fn from(val: BytesValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
