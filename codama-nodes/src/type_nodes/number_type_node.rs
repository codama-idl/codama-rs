use codama_nodes_derive::type_node;
use serde::{Deserialize, Serialize};

pub use NumberFormat::*;

#[type_node]
pub struct NumberTypeNode {
    // Data.
    pub format: NumberFormat,
    pub endian: Endian,
}

impl NumberTypeNode {
    pub fn new(format: NumberFormat, endian: Endian) -> Self {
        Self { format, endian }
    }

    pub fn le(format: NumberFormat) -> Self {
        Self::new(format, Endian::Little)
    }

    pub fn be(format: NumberFormat) -> Self {
        Self::new(format, Endian::Big)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NumberFormat {
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,
    ShortU16,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Endian {
    #[serde(rename = "be")]
    Big,
    #[serde(rename = "le")]
    Little,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = NumberTypeNode::new(U8, Endian::Big);
        assert_eq!(node.format, NumberFormat::U8);
        assert_eq!(node.endian, Endian::Big);
    }

    #[test]
    fn le() {
        let node = NumberTypeNode::le(U32);
        assert_eq!(node.format, NumberFormat::U32);
        assert_eq!(node.endian, Endian::Little);
    }

    #[test]
    fn be() {
        let node = NumberTypeNode::be(U32);
        assert_eq!(node.format, NumberFormat::U32);
        assert_eq!(node.endian, Endian::Big);
    }

    #[test]
    fn to_json() {
        let node = NumberTypeNode::be(U8);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"numberTypeNode","format":"u8","endian":"be"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"numberTypeNode","format":"u8","endian":"be"}"#;
        let node: NumberTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, NumberTypeNode::be(U8));
    }
}
