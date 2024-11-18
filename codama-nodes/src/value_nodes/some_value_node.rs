use crate::ValueNode;
use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
pub struct SomeValueNode {
    // Children.
    pub value: ValueNode,
}

impl SomeValueNode {
    pub fn new<T>(value: T) -> Self
    where
        T: Into<ValueNode>,
    {
        Self {
            value: value.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NumberValueNode;

    #[test]
    fn new() {
        let node = SomeValueNode::new(NumberValueNode::new(42));
        assert_eq!(node.value, ValueNode::Number(NumberValueNode::new(42)));
    }
}
