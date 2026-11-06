use crate::{CountNode, HasTransforms, PluginNode, TransformNode, TypeNode};
use codama_nodes_derive::type_node;

#[type_node]
pub struct SetTypeNode {
    // Children.
    pub item: Box<TypeNode>,
    pub count: Box<CountNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub transforms: Vec<TransformNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<SetTypeNode> for crate::Node {
    fn from(val: SetTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}

impl HasTransforms for SetTypeNode {
    fn transforms(&self) -> &[TransformNode] {
        &self.transforms
    }
    fn transforms_mut(&mut self) -> &mut Vec<TransformNode> {
        &mut self.transforms
    }
}
