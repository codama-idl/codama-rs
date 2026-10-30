use crate::{HasTransforms, IntegerTypeNode, PluginNode, TransformNode};
use codama_nodes_derive::type_node;

#[type_node]
pub struct BooleanTypeNode {
    // Children.
    pub size: IntegerTypeNode,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub transforms: Vec<TransformNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<BooleanTypeNode> for crate::Node {
    fn from(val: BooleanTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}

impl HasTransforms for BooleanTypeNode {
    fn transforms(&self) -> &[TransformNode] {
        &self.transforms
    }
    fn transforms_mut(&mut self) -> &mut Vec<TransformNode> {
        &mut self.transforms
    }
}
