use crate::PluginNode;
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct IdentityValueNode {
    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<IdentityValueNode> for crate::Node {
    fn from(val: IdentityValueNode) -> Self {
        crate::Node::ContextualValue(val.into())
    }
}
