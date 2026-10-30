use crate::PluginNode;
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct DataValueNode {
    // Data.
    pub path: String,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<DataValueNode> for crate::Node {
    fn from(val: DataValueNode) -> Self {
        crate::Node::ContextualValue(val.into())
    }
}
