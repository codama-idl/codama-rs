use crate::{NestedTypeNode, NumberTypeNode};
use codama_nodes_derive::type_node;

#[type_node]
pub struct AmountTypeNode {
    // Data.
    pub decimals: u32,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub unit: Option<String>,

    // Children.
    pub number: NestedTypeNode<NumberTypeNode>,
}

impl From<AmountTypeNode> for crate::Node {
    fn from(val: AmountTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
