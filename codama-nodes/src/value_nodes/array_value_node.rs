use super::ValueNode;
use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
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
}
