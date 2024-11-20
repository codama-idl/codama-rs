use crate::{TypeNode, ValueNode};
use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq, Clone)]
pub struct ConstantPdaSeedNode {
    // Children.
    pub r#type: TypeNode,
    pub value: ValueNode,
}

impl ConstantPdaSeedNode {
    pub fn new<T, U>(r#type: T, value: U) -> Self
    where
        T: Into<TypeNode>,
        U: Into<ValueNode>,
    {
        Self {
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NumberTypeNode, NumberValueNode, U64};

    #[test]
    fn new() {
        let node = ConstantPdaSeedNode::new(NumberTypeNode::le(U64), NumberValueNode::new(42u64));
        assert_eq!(node.r#type, TypeNode::Number(NumberTypeNode::le(U64)));
        assert_eq!(node.value, ValueNode::Number(NumberValueNode::new(42u64)));
    }
}
