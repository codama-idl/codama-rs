use crate::{CamelCaseString, PdaSeedValueNode, PdaSeedValueValue, ValueNode};

impl PdaSeedValueNode {
    pub fn new<T, U>(name: T, value: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<PdaSeedValueValue>,
    {
        Self {
            name: name.into(),
            value: Box::new(value.into()),
        }
    }
}

/// Bridge from the broader `ValueNode` union into `PdaSeedValueValue`.
/// `PdaSeedValueValue` is `accountValueNode | argumentValueNode | valueNode`
/// (17 variants total) — every value-node leaf maps to its same-named
/// `PdaSeedValueValue` variant.
impl From<ValueNode> for PdaSeedValueValue {
    fn from(value: ValueNode) -> Self {
        match value {
            ValueNode::Array(value) => Self::Array(value),
            ValueNode::Boolean(value) => Self::Boolean(value),
            ValueNode::Bytes(value) => Self::Bytes(value),
            ValueNode::Constant(value) => Self::Constant(value),
            ValueNode::Enum(value) => Self::Enum(value),
            ValueNode::Injected(value) => Self::Injected(value),
            ValueNode::Map(value) => Self::Map(value),
            ValueNode::None(value) => Self::None(value),
            ValueNode::Number(value) => Self::Number(value),
            ValueNode::PublicKey(value) => Self::PublicKey(value),
            ValueNode::Set(value) => Self::Set(value),
            ValueNode::Some(value) => Self::Some(value),
            ValueNode::String(value) => Self::String(value),
            ValueNode::Struct(value) => Self::Struct(value),
            ValueNode::Tuple(value) => Self::Tuple(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NumberValueNode;

    #[test]
    fn new() {
        let node = PdaSeedValueNode::new("answer", NumberValueNode::new(42));
        assert_eq!(node.name, CamelCaseString::from("answer"));
        assert_eq!(
            *node.value,
            PdaSeedValueValue::Number(NumberValueNode::new(42))
        );
    }

    #[test]
    fn to_json() {
        let node = PdaSeedValueNode::new("answer", NumberValueNode::new(42));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"pdaSeedValueNode","name":"answer","value":{"kind":"numberValueNode","number":42}}"#
        );
    }

    #[test]
    fn from_json() {
        let json: &str = r#"{"kind":"pdaSeedValueNode","name":"answer","value":{"kind":"numberValueNode","number":42}}"#;
        let node: PdaSeedValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            PdaSeedValueNode::new("answer", NumberValueNode::new(42u32))
        );
    }
}
