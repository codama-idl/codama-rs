use crate::{
    Endianness, HasTransforms, IntegerFormat, NumberDisplayNode, PluginNode, TransformNode,
};
use codama_nodes_derive::type_node;

#[type_node]
pub struct IntegerTypeNode {
    // Data.
    pub format: IntegerFormat,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub endian: Option<Endianness>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub unit: Option<String>,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub display: Box<Option<NumberDisplayNode>>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub transforms: Vec<TransformNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<IntegerTypeNode> for crate::Node {
    fn from(val: IntegerTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}

impl HasTransforms for IntegerTypeNode {
    fn transforms(&self) -> &[TransformNode] {
        &self.transforms
    }
    fn transforms_mut(&mut self) -> &mut Vec<TransformNode> {
        &mut self.transforms
    }
}
