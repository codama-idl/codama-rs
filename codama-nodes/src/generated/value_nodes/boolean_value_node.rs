use crate::PluginNode;
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct BooleanValueNode {
    // Data.
    pub boolean: bool,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<BooleanValueNode> for crate::Node {
    fn from(val: BooleanValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
