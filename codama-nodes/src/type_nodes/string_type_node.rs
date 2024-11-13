use codama_nodes_derive::{Node, TypeNode};

pub use BytesEncoding::*;

#[derive(Node, TypeNode, Debug)]
pub struct StringTypeNode {
    // Data.
    pub encoding: BytesEncoding,
}

impl StringTypeNode {
    pub fn new(encoding: BytesEncoding) -> Self {
        Self { encoding }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BytesEncoding {
    Base16,
    Base58,
    Base64,
    Utf8,
}
