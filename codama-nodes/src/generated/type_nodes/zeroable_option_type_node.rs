use crate::{ConstantValueNode, PluginNode, TransformNode, TypeNode};
use codama_nodes_derive::type_node;

#[type_node]
pub struct ZeroableOptionTypeNode {
    // Children.
    pub item: Box<TypeNode>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub zero_value: Option<ConstantValueNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub transforms: Vec<TransformNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<ZeroableOptionTypeNode> for crate::Node {
    fn from(val: ZeroableOptionTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
