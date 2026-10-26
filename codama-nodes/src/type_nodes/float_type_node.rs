use crate::{Endianness, FloatFormat, FloatTypeNode};
use codama_errors::{CodamaError, CodamaResult};

pub use FloatFormat::*;

impl FloatTypeNode {
    pub fn new(format: FloatFormat, endian: Endianness) -> Self {
        Self {
            format,
            endian: Some(endian),
            unit: None,
            display: None,
            transforms: vec![],
            plugins: vec![],
        }
    }

    pub fn le(format: FloatFormat) -> Self {
        Self::new(format, Endianness::Le)
    }

    pub fn be(format: FloatFormat) -> Self {
        Self::new(format, Endianness::Be)
    }
}

impl TryFrom<String> for FloatFormat {
    type Error = CodamaError;

    fn try_from(value: String) -> CodamaResult<Self> {
        value.as_str().try_into()
    }
}

impl TryFrom<&str> for FloatFormat {
    type Error = CodamaError;

    fn try_from(value: &str) -> CodamaResult<Self> {
        match value {
            "f32" => Ok(F32),
            "f64" => Ok(F64),
            _ => Err(CodamaError::InvalidNumberFormat(value.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = FloatTypeNode::new(F32, Endianness::Be);
        assert_eq!(node.format, FloatFormat::F32);
        assert_eq!(node.endian, Some(Endianness::Be));
        assert_eq!(node.unit, None);
        assert_eq!(node.display, None);
    }

    #[test]
    fn le() {
        let node = FloatTypeNode::le(F64);
        assert_eq!(node.format, FloatFormat::F64);
        assert_eq!(node.endian, Some(Endianness::Le));
    }

    #[test]
    fn be() {
        let node = FloatTypeNode::be(F64);
        assert_eq!(node.format, FloatFormat::F64);
        assert_eq!(node.endian, Some(Endianness::Be));
    }

    #[test]
    fn to_json() {
        let node = FloatTypeNode {
            unit: Some("USD".to_string()),
            ..FloatTypeNode::le(F64)
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"floatTypeNode","format":"f64","endian":"le","unit":"USD"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"floatTypeNode","format":"f32","endian":"be"}"#;
        let node: FloatTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, FloatTypeNode::new(F32, Endianness::Be));
    }

    #[test]
    fn float_format_try_from() {
        for (value, expected) in [("f32", F32), ("f64", F64)] {
            assert_eq!(FloatFormat::try_from(value).unwrap(), expected);
            assert_eq!(FloatFormat::try_from(value.to_string()).unwrap(), expected);
        }
    }

    #[test]
    fn float_format_try_from_rejects_other_formats() {
        for value in ["u32", "f16", "F32", ""] {
            let error = FloatFormat::try_from(value).unwrap_err();
            assert_eq!(error.to_string(), format!("Invalid number format: {value}"));
        }
    }

    #[test]
    fn from_json_without_endian() {
        let json = r#"{"kind":"floatTypeNode","format":"f64"}"#;
        let node: FloatTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node.endian, None);
        assert_eq!(serde_json::to_string(&node).unwrap(), json);
    }
}
