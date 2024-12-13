use crate::ValueNode;
use codama_nodes_derive::node;

#[node]
pub struct SomeValueNode {
    // Children.
    pub value: ValueNode,
}

impl From<SomeValueNode> for crate::Node {
    fn from(val: SomeValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}

impl SomeValueNode {
    pub fn new<T>(value: T) -> Self
    where
        T: Into<ValueNode>,
    {
        Self {
            value: value.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NumberValueNode;

    #[test]
    fn new() {
        let node = SomeValueNode::new(NumberValueNode::new(42));
        assert_eq!(node.value, ValueNode::Number(NumberValueNode::new(42)));
    }

    #[test]
    fn to_json() {
        let node = SomeValueNode::new(NumberValueNode::new(42));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"someValueNode","value":{"kind":"numberValueNode","number":42}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"someValueNode","value":{"kind":"numberValueNode","number":42}}"#;
        let node: SomeValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, SomeValueNode::new(NumberValueNode::new(42u32)));
    }
}
