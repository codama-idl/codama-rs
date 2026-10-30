use crate::{CountNode, HasTransforms, PluginNode, TransformNode, TypeNode};
use codama_nodes_derive::type_node;

#[type_node]
pub struct ArrayTypeNode {
    // Children.
    pub item: Box<TypeNode>,
    pub count: Box<CountNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub transforms: Vec<TransformNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<ArrayTypeNode> for crate::Node {
    fn from(val: ArrayTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}

impl HasTransforms for ArrayTypeNode {
    fn transforms(&self) -> &[TransformNode] {
        &self.transforms
    }
    fn transforms_mut(&mut self) -> &mut Vec<TransformNode> {
        &mut self.transforms
    }
}
