use crate::{Path, PluginNode};
use codama_nodes_derive::node;

#[node]
pub struct FieldDiscriminatorNode {
    // Data.
    pub path: Path,
    pub offset: u64,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<FieldDiscriminatorNode> for crate::Node {
    fn from(val: FieldDiscriminatorNode) -> Self {
        crate::Node::Discriminator(val.into())
    }
}
