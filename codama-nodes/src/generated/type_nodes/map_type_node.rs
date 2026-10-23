use crate::{CountNode, PluginNode, TransformNode, TypeNode};
use codama_nodes_derive::type_node;

#[type_node]
pub struct MapTypeNode {
    // Children.
    pub key: Box<TypeNode>,
    pub value: Box<TypeNode>,
    pub count: Box<CountNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub transforms: Vec<TransformNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<MapTypeNode> for crate::Node {
    fn from(val: MapTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
