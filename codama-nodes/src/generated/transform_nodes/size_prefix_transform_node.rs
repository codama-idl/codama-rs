use crate::{IntegerTypeNode, PluginNode};
use codama_nodes_derive::node;

#[node]
pub struct SizePrefixTransformNode {
    // Children.
    pub prefix: IntegerTypeNode,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<SizePrefixTransformNode> for crate::Node {
    fn from(val: SizePrefixTransformNode) -> Self {
        crate::Node::Transform(val.into())
    }
}
