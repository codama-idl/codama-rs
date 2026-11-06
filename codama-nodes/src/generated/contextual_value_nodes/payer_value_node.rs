use crate::PluginNode;
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct PayerValueNode {
    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<PayerValueNode> for crate::Node {
    fn from(val: PayerValueNode) -> Self {
        crate::Node::ContextualValue(val.into())
    }
}
