use crate::{MapEntryValueNode, ValueNode};

impl MapEntryValueNode {
    pub fn new<T, U>(key: T, value: U) -> Self
    where
        T: Into<ValueNode>,
        U: Into<ValueNode>,
    {
        Self {
            key: Box::new(key.into()),
            value: Box::new(value.into()),
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{IntegerValueNode, StringValueNode};

    #[test]
    fn new() {
        let node =
            MapEntryValueNode::new(StringValueNode::new("answer"), IntegerValueNode::new(42));
        assert_eq!(*node.key, ValueNode::String(StringValueNode::new("answer")));
        assert_eq!(*node.value, ValueNode::Integer(IntegerValueNode::new(42)));
    }

    #[test]
    fn to_json() {
        let node =
            MapEntryValueNode::new(StringValueNode::new("answer"), IntegerValueNode::new(42));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"mapEntryValueNode","key":{"kind":"stringValueNode","string":"answer"},"value":{"kind":"integerValueNode","value":"42"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"mapEntryValueNode","key":{"kind":"stringValueNode","string":"answer"},"value":{"kind":"integerValueNode","value":"42"}}"#;
        let node: MapEntryValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            MapEntryValueNode::new(StringValueNode::new("answer"), IntegerValueNode::new(42u32))
        );
    }
}
