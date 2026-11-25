use crate::{Identifier, Node, PdaSeedValueNode, PdaSeedValueValue, ValueNode};

impl PdaSeedValueNode {
    pub fn new<T, U>(identifier: T, value: U) -> Self
    where
        T: Into<Identifier>,
        U: Into<PdaSeedValueValue>,
    {
        Self {
            identifier: identifier.into(),
            value: Box::new(value.into()),
            plugins: vec![],
        }
    }
}

/// The spec union `pdaSeedValueValue` is `accountValueNode | dataValueNode | valueNode`, so every value
/// node is a member of it. Going through `Node` keeps this true whatever value nodes the spec adds.
impl From<ValueNode> for PdaSeedValueValue {
    fn from(value: ValueNode) -> Self {
        Self::try_from(Node::from(value)).expect("every value node is a member of the union")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DataValueNode, FloatValueNode, Identifier, IntegerValueNode, Path};

    #[test]
    fn new() {
        let node = PdaSeedValueNode::new("answer", IntegerValueNode::new(42));
        assert_eq!(node.identifier, Identifier::from("answer"));
        assert_eq!(
            *node.value,
            PdaSeedValueValue::IntegerValue(IntegerValueNode::new(42))
        );
    }

    #[test]
    fn new_with_data_value() {
        let path: Path = "amount".parse().unwrap();
        let node = PdaSeedValueNode::new("amount", DataValueNode::new(path.clone()));
        assert_eq!(
            *node.value,
            PdaSeedValueValue::DataValue(DataValueNode::new(path))
        );
    }

    #[test]
    fn from_value_node() {
        let value: ValueNode = FloatValueNode::new(1.5).into();
        assert_eq!(
            PdaSeedValueValue::from(value),
            PdaSeedValueValue::FloatValue(FloatValueNode::new(1.5))
        );
    }

    #[test]
    fn to_json() {
        let node = PdaSeedValueNode::new("answer", IntegerValueNode::new(42));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"pdaSeedValueNode","identifier":"answer","value":{"kind":"integerValueNode","value":"42"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json: &str = r#"{"kind":"pdaSeedValueNode","identifier":"answer","value":{"kind":"integerValueNode","value":"42"}}"#;
        let node: PdaSeedValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            PdaSeedValueNode::new("answer", IntegerValueNode::new(42u32))
        );
    }
}
