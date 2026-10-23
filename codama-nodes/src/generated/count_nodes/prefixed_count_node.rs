use crate::{IntegerTypeNode, PluginNode};
use codama_nodes_derive::node;

#[node]
pub struct PrefixedCountNode {
    // Children.
    pub prefix: IntegerTypeNode,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<PrefixedCountNode> for crate::Node {
    fn from(val: PrefixedCountNode) -> Self {
        crate::Node::Count(val.into())
    }
}
