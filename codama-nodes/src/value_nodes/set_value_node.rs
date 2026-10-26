use crate::{SetValueNode, ValueNode};

impl SetValueNode {
    pub fn new(items: Vec<ValueNode>) -> Self {
        Self {
            items,
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
        let node = SetValueNode::new(vec![
            IntegerValueNode::new(1).into(),
            IntegerValueNode::new(2).into(),
            IntegerValueNode::new(3).into(),
        ]);
        assert_eq!(
            node.items,
            vec![
                ValueNode::Integer(IntegerValueNode::new(1)),
                ValueNode::Integer(IntegerValueNode::new(2)),
                ValueNode::Integer(IntegerValueNode::new(3)),
            ]
        );
    }

    #[test]
    fn to_json() {
        let node = SetValueNode::new(vec![IntegerValueNode::new(42u32).into()]);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"setValueNode","items":[{"kind":"integerValueNode","value":"42"}]}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"setValueNode","items":[{"kind":"integerValueNode","value":"42"}]}"#;
        let node: SetValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            SetValueNode::new(vec![IntegerValueNode::new(42u32).into()])
        );
    }
}
