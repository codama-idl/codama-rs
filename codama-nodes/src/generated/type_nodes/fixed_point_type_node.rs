use crate::{
    Base, HasTransforms, IntegerTypeNode, PluginNode, TransformNode, UnitNumberDisplayNode,
};
use codama_nodes_derive::type_node;

#[type_node]
pub struct FixedPointTypeNode {
    // Data.
    pub scale: u32,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub base: Option<Base>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub unit: Option<String>,

    // Children.
    pub number: IntegerTypeNode,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub display: Option<UnitNumberDisplayNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub transforms: Vec<TransformNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<FixedPointTypeNode> for crate::Node {
    fn from(val: FixedPointTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}

impl HasTransforms for FixedPointTypeNode {
    fn transforms(&self) -> &[TransformNode] {
        &self.transforms
    }
    fn transforms_mut(&mut self) -> &mut Vec<TransformNode> {
        &mut self.transforms
    }
}
