use crate::StructFieldValueNode;
use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
pub struct StructValueNode {
    // Children.
    pub fields: Vec<StructFieldValueNode>,
}

impl StructValueNode {
    pub fn new(fields: Vec<StructFieldValueNode>) -> Self {
        Self { fields }
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
    use crate::{NumberValueNode, StringValueNode, ValueNode};

    #[test]
    fn new() {
        let node = StructValueNode::new(vec![
            StructFieldValueNode::new("name", StringValueNode::new("Alice")).into(),
            StructFieldValueNode::new("age", NumberValueNode::new(42)).into(),
        ]);
        assert_eq!(
            node.fields,
            vec![
                StructFieldValueNode::new("name", ValueNode::String(StringValueNode::new("Alice"))),
                StructFieldValueNode::new("age", ValueNode::Number(NumberValueNode::new(42))),
            ]
        );
    }

    #[test]
    fn from_vec() {
        let node: StructValueNode = vec![
            StructFieldValueNode::new("name", StringValueNode::new("Alice")).into(),
            StructFieldValueNode::new("age", NumberValueNode::new(42)).into(),
        ]
        .into();
        assert_eq!(
            node.fields,
            vec![
                StructFieldValueNode::new("name", ValueNode::String(StringValueNode::new("Alice"))),
                StructFieldValueNode::new("age", ValueNode::Number(NumberValueNode::new(42))),
            ]
        );
    }
}
