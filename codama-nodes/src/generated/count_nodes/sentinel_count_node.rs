use crate::{ConstantValueNode, PluginNode, SentinelCountStrategy};
use codama_nodes_derive::node;

#[node]
pub struct SentinelCountNode {
    // Data.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub strategy: Option<SentinelCountStrategy>,

    // Children.
    pub sentinel: ConstantValueNode,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<SentinelCountNode> for crate::Node {
    fn from(val: SentinelCountNode) -> Self {
        crate::Node::Count(val.into())
    }
}
