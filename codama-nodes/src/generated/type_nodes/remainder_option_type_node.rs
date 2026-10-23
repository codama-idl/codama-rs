use crate::{PluginNode, TransformNode, TypeNode};
use codama_nodes_derive::type_node;

#[type_node]
pub struct RemainderOptionTypeNode {
    // Children.
    pub item: Box<TypeNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub transforms: Vec<TransformNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<RemainderOptionTypeNode> for crate::Node {
    fn from(val: RemainderOptionTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
