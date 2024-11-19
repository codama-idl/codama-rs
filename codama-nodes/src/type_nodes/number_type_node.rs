use crate::NodeTrait;
use codama_nodes_derive::{Node, TypeNode};
use serde::{Deserialize, Serialize};

pub use NumberFormat::*;

#[derive(Node, TypeNode, Debug, PartialEq)]
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

// Implement Serialize to add the "kind" field.
#[derive(Serialize, Deserialize)]
struct SerdeWrapper<'a> {
    pub kind: &'a str,
    pub format: NumberFormat,
    pub endian: Endian,
}

impl Serialize for NumberTypeNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        SerdeWrapper {
            kind: Self::KIND,
            format: self.format,
            endian: self.endian,
        }
        .serialize(serializer)
    }
}

// Implement Deserialize to handle the "kind" field.
impl<'de> Deserialize<'de> for NumberTypeNode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let helper = SerdeWrapper::deserialize(deserializer)?;

        // Validate that the "kind" field matches the expected constant.
        if helper.kind != Self::KIND {
            return Err(serde::de::Error::custom(format!(
                "Invalid kind: expected '{}', got '{}'",
                Self::KIND,
                helper.kind
            )));
        }

        Ok(Self {
            format: helper.format,
            endian: helper.endian,
        })
    }
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
        let node = NumberTypeNode::new(U8, Endian::Big);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"numberTypeNode","format":"u8","endian":"be"}"#
        );
    }
}
