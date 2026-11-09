use crate::{BytesEncoding, PluginNode};
use codama_nodes_derive::node;

#[node]
pub struct BytesValueNode {
    pub data: String,
    pub encoding: BytesEncoding,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}
