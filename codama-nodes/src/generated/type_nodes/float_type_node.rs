use crate::{
    Endianness, FloatFormat, HasTransforms, PluginNode, TransformNode, UnitNumberDisplayNode,
};
use codama_nodes_derive::type_node;

#[type_node]
pub struct FloatTypeNode {
    // Data.
    pub format: FloatFormat,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub endian: Option<Endianness>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub unit: Option<String>,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub display: Option<UnitNumberDisplayNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub transforms: Vec<TransformNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<FloatTypeNode> for crate::Node {
    fn from(val: FloatTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}

impl HasTransforms for FloatTypeNode {
    fn transforms(&self) -> &[TransformNode] {
        &self.transforms
    }
    fn transforms_mut(&mut self) -> &mut Vec<TransformNode> {
        &mut self.transforms
    }
}
