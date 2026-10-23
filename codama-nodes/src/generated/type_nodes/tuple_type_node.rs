use crate::{PluginNode, TransformNode, TypeNode};
use codama_nodes_derive::type_node;

#[type_node]
#[derive(Default)]
pub struct TupleTypeNode {
    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub items: Vec<TypeNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub transforms: Vec<TransformNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<TupleTypeNode> for crate::Node {
    fn from(val: TupleTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
