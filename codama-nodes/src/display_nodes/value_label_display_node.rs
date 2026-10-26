use crate::{Text, ValueLabelDisplayNode, ValueNode};

impl ValueLabelDisplayNode {
    pub fn new<T, U>(value: T, label: U) -> Self
    where
        T: Into<ValueNode>,
        U: Into<Text>,
    {
        Self {
            value: Box::new(value.into()),
            label: label.into(),
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
        let node = ValueLabelDisplayNode::new(IntegerValueNode::new(1u8), "Active");
        assert_eq!(*node.value, ValueNode::Integer(IntegerValueNode::new(1u8)));
        assert_eq!(node.label, Text::from("Active"));
    }

    #[test]
    fn to_json() {
        let node = ValueLabelDisplayNode::new(IntegerValueNode::new(1u8), "Active");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"valueLabelDisplayNode","value":{"kind":"integerValueNode","value":"1"},"label":"Active"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"valueLabelDisplayNode","value":{"kind":"integerValueNode","value":"1"},"label":"Active"}"#;
        let node: ValueLabelDisplayNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            ValueLabelDisplayNode::new(IntegerValueNode::new(1u8), "Active")
        );
    }
}
