use crate::{SomeValueNode, ValueNode};

impl SomeValueNode {
    pub fn new<T>(value: T) -> Self
    where
        T: Into<ValueNode>,
    {
        Self {
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
        let node = SomeValueNode::new(IntegerValueNode::new(42));
        assert_eq!(*node.value, ValueNode::Integer(IntegerValueNode::new(42)));
    }

    #[test]
    fn to_json() {
        let node = SomeValueNode::new(IntegerValueNode::new(42));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"someValueNode","value":{"kind":"integerValueNode","value":"42"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"someValueNode","value":{"kind":"integerValueNode","value":"42"}}"#;
        let node: SomeValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, SomeValueNode::new(IntegerValueNode::new(42u32)));
    }
}
