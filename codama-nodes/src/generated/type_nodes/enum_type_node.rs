use crate::{EnumVariantTypeNode, IntegerTypeNode, PluginNode, TransformNode};
use codama_nodes_derive::type_node;

#[type_node]
pub struct EnumTypeNode {
    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub variants: Vec<EnumVariantTypeNode>,
    pub size: IntegerTypeNode,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub transforms: Vec<TransformNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<EnumTypeNode> for crate::Node {
    fn from(val: EnumTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
