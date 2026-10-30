use crate::{HasTransforms, IntegerTypeNode, PluginNode, TransformNode};
use codama_nodes_derive::type_node;

#[type_node]
pub struct DurationTypeNode {
    // Data.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub ticks_per_second: Option<u32>,

    // Children.
    pub number: IntegerTypeNode,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub transforms: Vec<TransformNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<DurationTypeNode> for crate::Node {
    fn from(val: DurationTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}

impl HasTransforms for DurationTypeNode {
    fn transforms(&self) -> &[TransformNode] {
        &self.transforms
    }
    fn transforms_mut(&mut self) -> &mut Vec<TransformNode> {
        &mut self.transforms
    }
}
