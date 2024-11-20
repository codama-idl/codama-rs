use crate::ValueNode;
use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq, Clone)]
pub struct MapEntryValueNode {
    // Children.
    pub key: ValueNode,
    pub value: ValueNode,
}

impl MapEntryValueNode {
    pub fn new<T, U>(key: T, value: U) -> Self
    where
        T: Into<ValueNode>,
        U: Into<ValueNode>,
    {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{NumberValueNode, StringValueNode};

    use super::*;

    #[test]
    fn new() {
        let node = MapEntryValueNode::new(StringValueNode::new("answer"), NumberValueNode::new(42));
        assert_eq!(node.key, ValueNode::String(StringValueNode::new("answer")));
        assert_eq!(node.value, ValueNode::Number(NumberValueNode::new(42)));
    }
}
