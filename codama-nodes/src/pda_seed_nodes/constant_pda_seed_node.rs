use crate::{ConstantPdaSeedNode, ConstantPdaSeedValue, TypeNode, ValueNode};

impl ConstantPdaSeedNode {
    pub fn new<T, U>(r#type: T, value: U) -> Self
    where
        T: Into<TypeNode>,
        U: Into<ConstantPdaSeedValue>,
    {
        Self {
            r#type: Box::new(r#type.into()),
            value: Box::new(value.into()),
            plugins: vec![],
        }
    }
}

/// Bridge from the broader value-node union into a `constantPdaSeedNode`
/// value. The spec union `constantPdaSeedValue` is `programIdValueNode |
/// valueNode`, so every `ValueNode` cleanly maps to its same-named
/// `ConstantPdaSeedValue` variant.
impl From<ValueNode> for ConstantPdaSeedValue {
    fn from(value: ValueNode) -> Self {
        match value {
            ValueNode::Array(n) => Self::Array(n),
            ValueNode::Boolean(n) => Self::Boolean(n),
            ValueNode::Bytes(n) => Self::Bytes(n),
            ValueNode::Constant(n) => Self::Constant(n),
            ValueNode::Enum(n) => Self::Enum(n),
            ValueNode::Float(n) => Self::Float(n),
            ValueNode::Injected(n) => Self::Injected(n),
            ValueNode::Integer(n) => Self::Integer(n),
            ValueNode::Map(n) => Self::Map(n),
            ValueNode::None(n) => Self::None(n),
            ValueNode::PublicKey(n) => Self::PublicKey(n),
            ValueNode::Set(n) => Self::Set(n),
            ValueNode::Some(n) => Self::Some(n),
            ValueNode::String(n) => Self::String(n),
            ValueNode::Struct(n) => Self::Struct(n),
            ValueNode::Tuple(n) => Self::Tuple(n),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{IntegerTypeNode, IntegerValueNode, U64};

    #[test]
    fn new() {
        let node = ConstantPdaSeedNode::new(IntegerTypeNode::le(U64), IntegerValueNode::new(42u64));
        assert_eq!(*node.r#type, TypeNode::Integer(IntegerTypeNode::le(U64)));
        assert_eq!(
            *node.value,
            ConstantPdaSeedValue::Integer(IntegerValueNode::new(42u64))
        );
    }

    #[test]
    fn to_json() {
        let node = ConstantPdaSeedNode::new(IntegerTypeNode::le(U64), IntegerValueNode::new(42u64));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"constantPdaSeedNode","type":{"kind":"integerTypeNode","format":"u64","endian":"le"},"value":{"kind":"integerValueNode","value":"42"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"constantPdaSeedNode","type":{"kind":"integerTypeNode","format":"u64","endian":"le"},"value":{"kind":"integerValueNode","value":"42"}}"#;
        let node: ConstantPdaSeedNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            ConstantPdaSeedNode::new(IntegerTypeNode::le(U64), IntegerValueNode::new(42u64))
        );
    }
}
