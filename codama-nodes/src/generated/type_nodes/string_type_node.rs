use crate::BytesEncoding;
use codama_nodes_derive::type_node;

#[type_node]
#[derive(Copy)]
pub struct StringTypeNode {
    // Data.
    pub encoding: BytesEncoding,
}

impl From<StringTypeNode> for crate::Node {
    fn from(val: StringTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
