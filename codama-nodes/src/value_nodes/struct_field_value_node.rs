use crate::{CamelCaseString, ValueNode};
use codama_nodes_derive::node;

#[node]
pub struct StructFieldValueNode {
    // Data.
    pub name: CamelCaseString,

    // Children.
    pub value: ValueNode,
}

impl StructFieldValueNode {
    pub fn new<T, U>(name: T, value: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<ValueNode>,
    {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::NumberValueNode;

    use super::*;

    #[test]
    fn new() {
        let node = StructFieldValueNode::new("answer", NumberValueNode::new(42));
        assert_eq!(node.name, CamelCaseString::from("answer"));
        assert_eq!(node.value, ValueNode::Number(NumberValueNode::new(42)));
    }
}
