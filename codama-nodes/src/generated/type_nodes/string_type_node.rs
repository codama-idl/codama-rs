use crate::{BytesEncoding, HasTransforms, PluginNode, StringDisplayNode, TransformNode};
use codama_nodes_derive::type_node;

#[type_node]
pub struct StringTypeNode {
    // Data.
    pub encoding: BytesEncoding,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub display: Option<StringDisplayNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub transforms: Vec<TransformNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<StringTypeNode> for crate::Node {
    fn from(val: StringTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}

impl HasTransforms for StringTypeNode {
    fn transforms(&self) -> &[TransformNode] {
        &self.transforms
    }
    fn transforms_mut(&mut self) -> &mut Vec<TransformNode> {
        &mut self.transforms
    }
}
