use crate::{HasTransforms, IntegerTypeNode, PluginNode, TransformNode, TypeNode};
use codama_nodes_derive::type_node;

#[type_node]
pub struct OptionTypeNode {
    #[serde(skip_serializing_if = "crate::is_default")]
    pub fixed: Option<bool>,
    pub item: Box<TypeNode>,
    pub prefix: IntegerTypeNode,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub transforms: Vec<TransformNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl HasTransforms for OptionTypeNode {
    fn transforms(&self) -> &[TransformNode] {
        &self.transforms
    }
    fn transforms_mut(&mut self) -> &mut Vec<TransformNode> {
        &mut self.transforms
    }
}
