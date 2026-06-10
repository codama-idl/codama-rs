use crate::{Endianness, NumberFormat};
use codama_nodes_derive::type_node;

#[type_node]
#[derive(Copy)]
pub struct NumberTypeNode {
    // Data.
    pub format: NumberFormat,
    pub endian: Endianness,
}

impl From<NumberTypeNode> for crate::Node {
    fn from(val: NumberTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
