use crate::{PluginNode, PreOffsetStrategy};
use codama_nodes_derive::node;

#[node]
pub struct PreOffsetTransformNode {
    // Data.
    pub offset: i64,
    pub strategy: PreOffsetStrategy,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<PreOffsetTransformNode> for crate::Node {
    fn from(val: PreOffsetTransformNode) -> Self {
        crate::Node::Transform(val.into())
    }
}
