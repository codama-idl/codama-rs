use crate::PluginNode;
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct FixedCountNode {
    pub value: u64,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<FixedCountNode> for crate::Node {
    fn from(val: FixedCountNode) -> Self {
        crate::Node::Count(val.into())
    }
}
