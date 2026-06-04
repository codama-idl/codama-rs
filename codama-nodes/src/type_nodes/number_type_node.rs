use codama_errors::{CodamaError, CodamaResult};
use codama_nodes_derive::type_node;
use serde::{Deserialize, Serialize};

pub use NumberFormat::*;

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

impl NumberTypeNode {
    pub fn new(format: NumberFormat, endian: Endianness) -> Self {
        Self { format, endian }
    }

    pub fn le(format: NumberFormat) -> Self {
        Self::new(format, Endianness::Little)
    }

    pub fn be(format: NumberFormat) -> Self {
        Self::new(format, Endianness::Big)
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

impl TryFrom<String> for NumberFormat {
    type Error = CodamaError;

    fn try_from(value: String) -> CodamaResult<Self> {
        value.as_str().try_into()
    }
}

impl TryFrom<&str> for NumberFormat {
    type Error = CodamaError;

    fn try_from(value: &str) -> CodamaResult<Self> {
        match value {
            "u8" => Ok(U8),
            "u16" => Ok(U16),
            "u32" => Ok(U32),
            "u64" => Ok(U64),
            "u128" => Ok(U128),
            "i8" => Ok(I8),
            "i16" => Ok(I16),
            "i32" => Ok(I32),
            "i64" => Ok(I64),
            "i128" => Ok(I128),
            "f32" => Ok(F32),
            "f64" => Ok(F64),
            "short_u16" => Ok(ShortU16),
            _ => Err(CodamaError::InvalidNumberFormat(value.to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Endianness {
    #[serde(rename = "be")]
    Big,
    #[serde(rename = "le")]
    Little,
}

impl TryFrom<String> for Endianness {
    type Error = CodamaError;

    fn try_from(value: String) -> CodamaResult<Self> {
        value.as_str().try_into()
    }
}

impl TryFrom<&str> for Endianness {
    type Error = CodamaError;

    fn try_from(value: &str) -> CodamaResult<Self> {
        match value {
            "be" | "big" => Ok(Endianness::Big),
            "le" | "little" => Ok(Endianness::Little),
            _ => Err(CodamaError::InvalidEndian(value.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = NumberTypeNode::new(U8, Endianness::Big);
        assert_eq!(node.format, NumberFormat::U8);
        assert_eq!(node.endian, Endianness::Big);
    }

    #[test]
    fn le() {
        let node = NumberTypeNode::le(U32);
        assert_eq!(node.format, NumberFormat::U32);
        assert_eq!(node.endian, Endianness::Little);
    }

    #[test]
    fn be() {
        let node = NumberTypeNode::be(U32);
        assert_eq!(node.format, NumberFormat::U32);
        assert_eq!(node.endian, Endianness::Big);
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
