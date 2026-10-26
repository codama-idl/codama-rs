use crate::{Endianness, IntegerFormat, IntegerTypeNode};
use codama_errors::{CodamaError, CodamaResult};

pub use IntegerFormat::*;

impl IntegerTypeNode {
    pub fn new(format: IntegerFormat, endian: Endianness) -> Self {
        Self {
            format,
            endian: Some(endian),
            unit: None,
            display: Box::new(None),
            transforms: vec![],
            plugins: vec![],
        }
    }

    pub fn le(format: IntegerFormat) -> Self {
        Self::new(format, Endianness::Le)
    }

    pub fn be(format: IntegerFormat) -> Self {
        Self::new(format, Endianness::Be)
    }
}

impl TryFrom<String> for IntegerFormat {
    type Error = CodamaError;

    fn try_from(value: String) -> CodamaResult<Self> {
        value.as_str().try_into()
    }
}

impl TryFrom<&str> for IntegerFormat {
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
            "short_u16" => Ok(ShortU16),
            _ => Err(CodamaError::InvalidNumberFormat(value.to_string())),
        }
    }
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
            "be" | "big" => Ok(Endianness::Be),
            "le" | "little" => Ok(Endianness::Le),
            _ => Err(CodamaError::InvalidEndian(value.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = IntegerTypeNode::new(U8, Endianness::Be);
        assert_eq!(node.format, IntegerFormat::U8);
        assert_eq!(node.endian, Some(Endianness::Be));
    }

    #[test]
    fn le() {
        let node = IntegerTypeNode::le(U32);
        assert_eq!(node.format, IntegerFormat::U32);
        assert_eq!(node.endian, Some(Endianness::Le));
    }

    #[test]
    fn be() {
        let node = IntegerTypeNode::be(U32);
        assert_eq!(node.format, IntegerFormat::U32);
        assert_eq!(node.endian, Some(Endianness::Be));
    }

    #[test]
    fn to_json() {
        let node = IntegerTypeNode::le(ShortU16);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"integerTypeNode","format":"shortU16","endian":"le"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"integerTypeNode","format":"u8","endian":"be"}"#;
        let node: IntegerTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, IntegerTypeNode::be(U8));
    }

    #[test]
    fn integer_format_try_from() {
        for (value, expected) in [
            ("u8", U8),
            ("u16", U16),
            ("u32", U32),
            ("u64", U64),
            ("u128", U128),
            ("i8", I8),
            ("i16", I16),
            ("i32", I32),
            ("i64", I64),
            ("i128", I128),
            ("short_u16", ShortU16),
        ] {
            assert_eq!(IntegerFormat::try_from(value).unwrap(), expected);
            assert_eq!(
                IntegerFormat::try_from(value.to_string()).unwrap(),
                expected
            );
        }
    }

    #[test]
    fn integer_format_try_from_rejects_other_formats() {
        for value in ["f32", "f64", "u256", "U8", ""] {
            let error = IntegerFormat::try_from(value).unwrap_err();
            assert_eq!(error.to_string(), format!("Invalid number format: {value}"));
        }
    }

    #[test]
    fn endianness_try_from() {
        for (value, expected) in [
            ("be", Endianness::Be),
            ("big", Endianness::Be),
            ("le", Endianness::Le),
            ("little", Endianness::Le),
        ] {
            assert_eq!(Endianness::try_from(value).unwrap(), expected);
            assert_eq!(Endianness::try_from(value.to_string()).unwrap(), expected);
        }
    }

    #[test]
    fn endianness_try_from_rejects_other_values() {
        let error = Endianness::try_from("middle").unwrap_err();
        assert_eq!(error.to_string(), "Invalid endian: middle");
    }

    #[test]
    fn from_json_without_endian() {
        let json = r#"{"kind":"integerTypeNode","format":"u64"}"#;
        let node: IntegerTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node.endian, None);
        assert_eq!(serde_json::to_string(&node).unwrap(), json);
    }
}
