use crate::ValueNode;
use codama_nodes_derive::node;

#[node]
pub struct TupleValueNode {
    // Children.
    pub items: Vec<ValueNode>,
}

impl TupleValueNode {
    pub fn new(items: Vec<ValueNode>) -> Self {
        Self { items }
    }
}

impl From<Vec<ValueNode>> for TupleValueNode {
    fn from(items: Vec<ValueNode>) -> Self {
        Self::new(items)
    }
}

#[cfg(test)]
mod tests {
    use crate::{BooleanValueNode, NumberValueNode, StringValueNode};

    use super::*;

    #[test]
    fn new() {
        let node = TupleValueNode::new(vec![
            NumberValueNode::new(42).into(),
            StringValueNode::new("Hello").into(),
            BooleanValueNode::new(true).into(),
        ]);
        assert_eq!(
            node.items,
            vec![
                ValueNode::Number(NumberValueNode::new(42)),
                ValueNode::String(StringValueNode::new("Hello")),
                ValueNode::Boolean(BooleanValueNode::new(true)),
            ]
        );
    }

    #[test]
    fn from_vec() {
        let node: TupleValueNode = vec![NumberValueNode::new(42).into()].into();
        assert_eq!(
            node.items,
            vec![ValueNode::Number(NumberValueNode::new(42))]
        );
    }

    #[test]
    fn to_json() {
        let node = TupleValueNode::new(vec![NumberValueNode::new(42).into()]);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"tupleValueNode","items":[{"kind":"numberValueNode","number":42}]}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"tupleValueNode","items":[{"kind":"numberValueNode","number":42}]}"#;
        let node: TupleValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            TupleValueNode::new(vec![NumberValueNode::new(42u32).into()])
        );
    }
}
