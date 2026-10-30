use crate::{ConstantValueNode, PluginNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct HiddenSuffixTransformNode {
    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub suffix: Vec<ConstantValueNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<HiddenSuffixTransformNode> for crate::Node {
    fn from(val: HiddenSuffixTransformNode) -> Self {
        crate::Node::Transform(val.into())
    }
}
