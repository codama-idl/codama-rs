use crate::{PluginNode, PreOffsetStrategy};
use codama_nodes_derive::node;

#[node]
pub struct PreOffsetTransformNode {
    pub offset: i64,
    pub strategy: PreOffsetStrategy,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}
