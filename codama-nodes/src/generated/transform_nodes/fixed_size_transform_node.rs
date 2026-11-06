use crate::PluginNode;
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct FixedSizeTransformNode {
    // Data.
    pub size: u64,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<FixedSizeTransformNode> for crate::Node {
    fn from(val: FixedSizeTransformNode) -> Self {
        crate::Node::Transform(val.into())
    }
}
