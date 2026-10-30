use crate::PluginNode;
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct NoneValueNode {
    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<NoneValueNode> for crate::Node {
    fn from(val: NoneValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
