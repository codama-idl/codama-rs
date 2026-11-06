use crate::{HasTransforms, PluginNode, TransformNode};
use codama_nodes_derive::type_node;

#[type_node]
#[derive(Default)]
pub struct BytesTypeNode {
    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub transforms: Vec<TransformNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<BytesTypeNode> for crate::Node {
    fn from(val: BytesTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}

impl HasTransforms for BytesTypeNode {
    fn transforms(&self) -> &[TransformNode] {
        &self.transforms
    }
    fn transforms_mut(&mut self) -> &mut Vec<TransformNode> {
        &mut self.transforms
    }
}
