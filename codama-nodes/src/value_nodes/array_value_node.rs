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
    use super::*;

    #[test]
    fn new() {
        let node = ArrayValueNode::new(vec![]); // TODO: Add items.
        assert_eq!(node.items, vec![]);
    }
}
