use crate::PluginNode;
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct StringValueNode {
    // Data.
    pub string: String,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<StringValueNode> for crate::Node {
    fn from(val: StringValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
