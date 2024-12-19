use codama_nodes_derive::node;
use derive_more::derive::From;
use serde::{Deserialize, Serialize};
use serde_json::Number as JsonNumber;

#[node]
pub struct NumberValueNode {
    // Data.
    pub number: Number,
}

impl From<NumberValueNode> for crate::Node {
    fn from(val: NumberValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}

impl NumberValueNode {
    pub fn new<T>(number: T) -> Self
    where
        T: Into<Number>,
    {
        Self {
            number: number.into(),
        }
    }
}

#[derive(From, Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(from = "JsonNumber", into = "JsonNumber")]
pub enum Number {
    UnsignedInteger(u64),
    SignedInteger(i64),
    Float(f64),
}

impl From<u8> for Number {
    fn from(number: u8) -> Self {
        Number::UnsignedInteger(number as u64)
    }
}

impl From<u16> for Number {
    fn from(number: u16) -> Self {
        Number::UnsignedInteger(number as u64)
    }
}

impl From<u32> for Number {
    fn from(number: u32) -> Self {
        Number::UnsignedInteger(number as u64)
    }
}

impl From<i8> for Number {
    fn from(number: i8) -> Self {
        Number::SignedInteger(number as i64)
    }
}

impl From<i16> for Number {
    fn from(number: i16) -> Self {
        Number::SignedInteger(number as i64)
    }
}

impl From<i32> for Number {
    fn from(number: i32) -> Self {
        Number::SignedInteger(number as i64)
    }
}

impl From<f32> for Number {
    fn from(number: f32) -> Self {
        Number::Float(number as f64)
    }
}

impl From<JsonNumber> for Number {
    fn from(number: JsonNumber) -> Self {
        if number.is_u64() {
            Number::UnsignedInteger(number.as_u64().unwrap())
        } else if number.is_i64() {
            Number::SignedInteger(number.as_i64().unwrap())
        } else {
            Number::Float(number.as_f64().unwrap())
        }
    }
}

impl From<Number> for JsonNumber {
    fn from(val: Number) -> Self {
        match val {
            Number::UnsignedInteger(number) => JsonNumber::from(number),
            Number::SignedInteger(number) => JsonNumber::from(number),
            Number::Float(number) => JsonNumber::from_f64(number).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        assert_eq!(NumberValueNode::new(42).number, Number::SignedInteger(42));
        assert_eq!(
            NumberValueNode::new(42u8).number,
            Number::UnsignedInteger(42)
        );
        assert_eq!(
            NumberValueNode::new(42u16).number,
            Number::UnsignedInteger(42)
        );
        assert_eq!(
            NumberValueNode::new(42u32).number,
            Number::UnsignedInteger(42)
        );
        assert_eq!(
            NumberValueNode::new(42u64).number,
            Number::UnsignedInteger(42)
        );
        assert_eq!(
            NumberValueNode::new(-42i8).number,
            Number::SignedInteger(-42)
        );
        assert_eq!(
            NumberValueNode::new(-42i16).number,
            Number::SignedInteger(-42)
        );
        assert_eq!(
            NumberValueNode::new(-42i32).number,
            Number::SignedInteger(-42)
        );
        assert_eq!(
            NumberValueNode::new(-42i64).number,
            Number::SignedInteger(-42)
        );
        assert_eq!(NumberValueNode::new(1.5).number, Number::Float(1.5));
        assert_eq!(NumberValueNode::new(1.5f32).number, Number::Float(1.5));
        assert_eq!(NumberValueNode::new(1.5f64).number, Number::Float(1.5));
    }

    #[test]
    fn to_json() {
        let node = NumberValueNode::new(42u16);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"numberValueNode","number":42}"#);
    }

    #[test]
    fn from_json() {
        let node: NumberValueNode =
            serde_json::from_str(r#"{"kind":"numberValueNode","number":42}"#).unwrap();
        assert_eq!(node.number, Number::UnsignedInteger(42));
    }
}
