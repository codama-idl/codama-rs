use crate::{Endianness, NumberDisplayNode, NumberFormat};
use codama_nodes_derive::type_node;

#[type_node]
pub struct NumberTypeNode {
    // Data.
    pub format: NumberFormat,
    pub endian: Endianness,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub display: Box<Option<NumberDisplayNode>>,
}

impl From<NumberTypeNode> for crate::Node {
    fn from(val: NumberTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
