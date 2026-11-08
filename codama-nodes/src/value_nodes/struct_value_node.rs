use crate::{StructFieldValueNode, StructValueNode};

impl StructValueNode {
    pub fn new(fields: Vec<StructFieldValueNode>) -> Self {
        Self {
            fields,
            plugins: vec![],
        }
    }
}

impl From<Vec<StructFieldValueNode>> for StructValueNode {
    fn from(items: Vec<StructFieldValueNode>) -> Self {
        Self::new(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{IntegerValueNode, StringValueNode, ValueNode};

    #[test]
    fn new() {
        let node = StructValueNode::new(vec![
            StructFieldValueNode::new("name", StringValueNode::new("Alice")),
            StructFieldValueNode::new("age", IntegerValueNode::new(42)),
        ]);
        assert_eq!(
            node.fields,
            vec![
                StructFieldValueNode::new("name", ValueNode::String(StringValueNode::new("Alice"))),
                StructFieldValueNode::new("age", ValueNode::Integer(IntegerValueNode::new(42))),
            ]
        );
    }

    #[test]
    fn from_vec() {
        let node: StructValueNode = vec![
            StructFieldValueNode::new("name", StringValueNode::new("Alice")),
            StructFieldValueNode::new("age", IntegerValueNode::new(42)),
        ]
        .into();
        assert_eq!(
            node.fields,
            vec![
                StructFieldValueNode::new("name", ValueNode::String(StringValueNode::new("Alice"))),
                StructFieldValueNode::new("age", ValueNode::Integer(IntegerValueNode::new(42))),
            ]
        );
    }

    #[test]
    fn to_json() {
        let node = StructValueNode::new(vec![StructFieldValueNode::new(
            "answer",
            IntegerValueNode::new(42),
        )]);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"structValueNode","fields":[{"kind":"structFieldValueNode","identifier":"answer","value":{"kind":"integerValueNode","value":"42"}}]}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"structValueNode","fields":[{"kind":"structFieldValueNode","identifier":"answer","value":{"kind":"integerValueNode","value":"42"}}]}"#;
        let node: StructValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            StructValueNode::new(vec![StructFieldValueNode::new(
                "answer",
                IntegerValueNode::new(42u32),
            )])
        );
    }
}
