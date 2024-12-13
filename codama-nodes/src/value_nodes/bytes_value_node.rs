use crate::BytesEncoding;
use codama_nodes_derive::node;

#[node]
pub struct BytesValueNode {
    // Data.
    pub data: String,
    pub encoding: BytesEncoding,
}

impl From<BytesValueNode> for crate::Node {
    fn from(val: BytesValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}

impl BytesValueNode {
    pub fn new<T>(encoding: BytesEncoding, data: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            encoding,
            data: data.into(),
        }
    }

    pub fn base16<T>(data: T) -> Self
    where
        T: Into<String>,
    {
        Self::new(BytesEncoding::Base16, data)
    }

    pub fn base58<T>(data: T) -> Self
    where
        T: Into<String>,
    {
        Self::new(BytesEncoding::Base58, data)
    }

    pub fn base64<T>(data: T) -> Self
    where
        T: Into<String>,
    {
        Self::new(BytesEncoding::Base64, data)
    }

    pub fn utf8<T>(data: T) -> Self
    where
        T: Into<String>,
    {
        Self::new(BytesEncoding::Utf8, data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Utf8;

    #[test]
    fn new() {
        let node = BytesValueNode::new(Utf8, "Hello World");
        assert_eq!(node.encoding, BytesEncoding::Utf8);
        assert_eq!(node.data, "Hello World");
    }

    #[test]
    fn base16() {
        let node = BytesValueNode::base16("deadb0d1e5");
        assert_eq!(node.encoding, BytesEncoding::Base16);
        assert_eq!(node.data, "deadb0d1e5");
    }

    #[test]
    fn base58() {
        let node = BytesValueNode::base58("AoxAdTcWDxzTkzJf");
        assert_eq!(node.encoding, BytesEncoding::Base58);
        assert_eq!(node.data, "AoxAdTcWDxzTkzJf");
    }

    #[test]
    fn base64() {
        let node = BytesValueNode::base64("HelloWorld++");
        assert_eq!(node.encoding, BytesEncoding::Base64);
        assert_eq!(node.data, "HelloWorld++");
    }

    #[test]
    fn utf8() {
        let node = BytesValueNode::utf8("Hello World");
        assert_eq!(node.encoding, BytesEncoding::Utf8);
        assert_eq!(node.data, "Hello World");
    }

    #[test]
    fn to_json() {
        let node = BytesValueNode::base16("deadb0d1e5");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"bytesValueNode","data":"deadb0d1e5","encoding":"base16"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"bytesValueNode","data":"deadb0d1e5","encoding":"base16"}"#;
        let node: BytesValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, BytesValueNode::base16("deadb0d1e5"));
    }
}
