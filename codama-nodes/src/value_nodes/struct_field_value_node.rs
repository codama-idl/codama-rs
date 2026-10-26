use crate::{CamelCaseString, StructFieldValueNode, ValueNode};

impl StructFieldValueNode {
    pub fn new<T, U>(identifier: T, value: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<ValueNode>,
    {
        Self {
            identifier: identifier.into(),
            value: Box::new(value.into()),
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::IntegerValueNode;

    #[test]
    fn new() {
        let node = StructFieldValueNode::new("answer", IntegerValueNode::new(42));
        assert_eq!(node.identifier, CamelCaseString::from("answer"));
        assert_eq!(*node.value, ValueNode::Integer(IntegerValueNode::new(42)));
    }

    #[test]
    fn to_json() {
        let node = StructFieldValueNode::new("answer", IntegerValueNode::new(42));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"structFieldValueNode","identifier":"answer","value":{"kind":"integerValueNode","value":"42"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"structFieldValueNode","identifier":"answer","value":{"kind":"integerValueNode","value":"42"}}"#;
        let node: StructFieldValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            StructFieldValueNode::new("answer", IntegerValueNode::new(42u32))
        );
    }
}
