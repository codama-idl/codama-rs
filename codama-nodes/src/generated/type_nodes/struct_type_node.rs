use crate::{HasTransforms, PluginNode, StructFieldTypeNode, TransformNode};
use codama_nodes_derive::type_node;

#[type_node]
#[derive(Default)]
pub struct StructTypeNode {
    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub fields: Vec<StructFieldTypeNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub transforms: Vec<TransformNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<StructTypeNode> for crate::Node {
    fn from(val: StructTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}

impl HasTransforms for StructTypeNode {
    fn transforms(&self) -> &[TransformNode] {
        &self.transforms
    }
    fn transforms_mut(&mut self) -> &mut Vec<TransformNode> {
        &mut self.transforms
    }
}
