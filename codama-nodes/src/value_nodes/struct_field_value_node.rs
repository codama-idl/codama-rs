use crate::{CamelCaseString, ValueNode};
use codama_nodes_derive::node;

#[node]
pub struct StructFieldValueNode {
    // Data.
    pub name: CamelCaseString,

    // Children.
    pub value: ValueNode,
}

impl From<StructFieldValueNode> for crate::Node {
    fn from(val: StructFieldValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}

impl StructFieldValueNode {
    pub fn new<T, U>(name: T, value: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<ValueNode>,
    {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::NumberValueNode;

    use super::*;

    #[test]
    fn new() {
        let node = StructFieldValueNode::new("answer", NumberValueNode::new(42));
        assert_eq!(node.name, CamelCaseString::from("answer"));
        assert_eq!(node.value, ValueNode::Number(NumberValueNode::new(42)));
    }

    #[test]
    fn to_json() {
        let node = StructFieldValueNode::new("answer", NumberValueNode::new(42));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"structFieldValueNode","name":"answer","value":{"kind":"numberValueNode","number":42}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"structFieldValueNode","name":"answer","value":{"kind":"numberValueNode","number":42}}"#;
        let node: StructFieldValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            StructFieldValueNode::new("answer", NumberValueNode::new(42u32))
        );
    }
}
