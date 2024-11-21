use crate::ValueNode;
use codama_nodes_derive::node;

#[node]
pub struct SetValueNode {
    // Children.
    pub items: Vec<ValueNode>,
}

impl SetValueNode {
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
        let node = SetValueNode::new(vec![
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
