use crate::{NestedTypeNode, NumberTypeNode, TypeNode};
use codama_nodes_derive::type_node;

#[type_node]
pub struct OptionTypeNode {
    // Data.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub fixed: Option<bool>,

    // Children.
    pub item: Box<TypeNode>,
    pub prefix: NestedTypeNode<NumberTypeNode>,
}

impl From<OptionTypeNode> for crate::Node {
    fn from(val: OptionTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
