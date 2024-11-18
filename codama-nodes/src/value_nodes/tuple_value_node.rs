use super::ValueNode;
use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
pub struct TupleValueNode {
    // Children.
    pub items: Vec<ValueNode>,
}

impl TupleValueNode {
    pub fn new(items: Vec<ValueNode>) -> Self {
        Self { items }
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
}
