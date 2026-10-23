use crate::{ConstantValueNode, PluginNode};
use codama_nodes_derive::node;

#[node]
pub struct ConstantDiscriminatorNode {
    // Data.
    pub offset: u64,

    // Children.
    pub constant: ConstantValueNode,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<ConstantDiscriminatorNode> for crate::Node {
    fn from(val: ConstantDiscriminatorNode) -> Self {
        crate::Node::Discriminator(val.into())
    }
}
