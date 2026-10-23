use crate::{ConstantValueNode, PluginNode};
use codama_nodes_derive::node;

#[node]
pub struct SentinelTransformNode {
    // Children.
    pub sentinel: ConstantValueNode,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<SentinelTransformNode> for crate::Node {
    fn from(val: SentinelTransformNode) -> Self {
        crate::Node::Transform(val.into())
    }
}
