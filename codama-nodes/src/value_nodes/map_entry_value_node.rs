use crate::ValueNode;
use codama_nodes_derive::node;

#[node]
pub struct MapEntryValueNode {
    // Children.
    pub key: ValueNode,
    pub value: ValueNode,
}

impl Into<crate::Node> for MapEntryValueNode {
    fn into(self) -> crate::Node {
        crate::Node::Value(self.into())
    }
}

impl MapEntryValueNode {
    pub fn new<T, U>(key: T, value: U) -> Self
    where
        T: Into<ValueNode>,
        U: Into<ValueNode>,
    {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{NumberValueNode, StringValueNode};

    use super::*;

    #[test]
    fn new() {
        let node = MapEntryValueNode::new(StringValueNode::new("answer"), NumberValueNode::new(42));
        assert_eq!(node.key, ValueNode::String(StringValueNode::new("answer")));
        assert_eq!(node.value, ValueNode::Number(NumberValueNode::new(42)));
    }

    #[test]
    fn to_json() {
        let node = MapEntryValueNode::new(StringValueNode::new("answer"), NumberValueNode::new(42));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"mapEntryValueNode","key":{"kind":"stringValueNode","string":"answer"},"value":{"kind":"numberValueNode","number":42}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"mapEntryValueNode","key":{"kind":"stringValueNode","string":"answer"},"value":{"kind":"numberValueNode","number":42}}"#;
        let node: MapEntryValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            MapEntryValueNode::new(StringValueNode::new("answer"), NumberValueNode::new(42u32))
        );
    }
}
