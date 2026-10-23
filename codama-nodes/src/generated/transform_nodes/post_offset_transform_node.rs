use crate::{PluginNode, PostOffsetStrategy};
use codama_nodes_derive::node;

#[node]
pub struct PostOffsetTransformNode {
    // Data.
    pub offset: i64,
    pub strategy: PostOffsetStrategy,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<PostOffsetTransformNode> for crate::Node {
    fn from(val: PostOffsetTransformNode) -> Self {
        crate::Node::Transform(val.into())
    }
}
