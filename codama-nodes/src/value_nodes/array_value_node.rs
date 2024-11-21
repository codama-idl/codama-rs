use crate::ValueNode;
use codama_nodes_derive::node;

#[node]
pub struct ArrayValueNode {
    // Children.
    pub items: Vec<ValueNode>,
}

impl ArrayValueNode {
    pub fn new(items: Vec<ValueNode>) -> Self {
        Self { items }
    }
}

#[cfg(test)]
mod tests {
    use crate::NumberValueNode;

    use super::*;

    #[test]
    fn new() {
        let node = ArrayValueNode::new(vec![
            NumberValueNode::new(1).into(),
            NumberValueNode::new(2).into(),
            NumberValueNode::new(3).into(),
        ]);
        assert_eq!(
            node.items,
            vec![
                ValueNode::Number(NumberValueNode::new(1)),
                ValueNode::Number(NumberValueNode::new(2)),
                ValueNode::Number(NumberValueNode::new(3)),
            ]
        );
    }

    #[test]
    fn to_json() {
        let node = ArrayValueNode::new(vec![NumberValueNode::new(42u32).into()]);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"arrayValueNode","items":[{"kind":"numberValueNode","number":42}]}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"arrayValueNode","items":[{"kind":"numberValueNode","number":42}]}"#;
        let node: ArrayValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            ArrayValueNode::new(vec![NumberValueNode::new(42u32).into()])
        );
    }
}
