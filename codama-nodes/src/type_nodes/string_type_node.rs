use crate::BytesEncoding;
use codama_nodes_derive::{Node, TypeNode};

#[derive(Node, TypeNode, Debug, PartialEq)]
pub struct StringTypeNode {
    // Data.
    pub encoding: BytesEncoding,
}

impl StringTypeNode {
    pub fn new(encoding: BytesEncoding) -> Self {
        Self { encoding }
    }

    pub fn base16() -> Self {
        Self::new(BytesEncoding::Base16)
    }

    pub fn base58() -> Self {
        Self::new(BytesEncoding::Base58)
    }

    pub fn base64() -> Self {
        Self::new(BytesEncoding::Base64)
    }

    pub fn utf8() -> Self {
        Self::new(BytesEncoding::Utf8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Utf8;

    #[test]
    fn new() {
        let node = StringTypeNode::new(Utf8);
        assert_eq!(node.encoding, BytesEncoding::Utf8);
    }

    #[test]
    fn base16() {
        let node = StringTypeNode::base16();
        assert_eq!(node.encoding, BytesEncoding::Base16);
    }

    #[test]
    fn base58() {
        let node = StringTypeNode::base58();
        assert_eq!(node.encoding, BytesEncoding::Base58);
    }

    #[test]
    fn base64() {
        let node = StringTypeNode::base64();
        assert_eq!(node.encoding, BytesEncoding::Base64);
    }

    #[test]
    fn utf8() {
        let node = StringTypeNode::utf8();
        assert_eq!(node.encoding, BytesEncoding::Utf8);
    }
}
