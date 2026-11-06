use crate::PluginNode;
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct RemainderCountNode {
    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<RemainderCountNode> for crate::Node {
    fn from(val: RemainderCountNode) -> Self {
        crate::Node::Count(val.into())
    }
}
