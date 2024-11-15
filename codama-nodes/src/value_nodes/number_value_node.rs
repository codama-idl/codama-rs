use codama_nodes_derive::{IntoEnum, Node};

#[derive(Node, Debug, PartialEq)]
pub struct NumberValueNode {
    // Data.
    pub number: NumberValue,
}

impl NumberValueNode {
    pub fn new<T>(number: T) -> Self
    where
        T: Into<NumberValue>,
    {
        Self {
            number: number.into(),
        }
    }
}

#[derive(IntoEnum, Debug, PartialEq, Clone, Copy)]
pub enum NumberValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        assert_eq!(NumberValueNode::new(42).number, NumberValue::I32(42));
        assert_eq!(NumberValueNode::new(42u8).number, NumberValue::U8(42));
        assert_eq!(NumberValueNode::new(42u16).number, NumberValue::U16(42));
        assert_eq!(NumberValueNode::new(42u32).number, NumberValue::U32(42));
        assert_eq!(NumberValueNode::new(42u64).number, NumberValue::U64(42));
        assert_eq!(NumberValueNode::new(-42i8).number, NumberValue::I8(-42));
        assert_eq!(NumberValueNode::new(-42i16).number, NumberValue::I16(-42));
        assert_eq!(NumberValueNode::new(-42i32).number, NumberValue::I32(-42));
        assert_eq!(NumberValueNode::new(-42i64).number, NumberValue::I64(-42));
        assert_eq!(NumberValueNode::new(1.5).number, NumberValue::F64(1.5));
        assert_eq!(NumberValueNode::new(1.5f32).number, NumberValue::F32(1.5));
        assert_eq!(NumberValueNode::new(1.5f64).number, NumberValue::F64(1.5));
    }
}
