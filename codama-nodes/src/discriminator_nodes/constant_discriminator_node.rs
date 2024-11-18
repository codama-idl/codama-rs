use crate::ConstantValueNode;
use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
pub struct ConstantDiscriminatorNode {
    // Data.
    pub offset: usize,

    // Children.
    pub constant: ConstantValueNode,
}

impl ConstantDiscriminatorNode {
    pub fn new<T>(constant: T, offset: usize) -> Self
    where
        T: Into<ConstantValueNode>,
    {
        Self {
            constant: constant.into(),
            offset,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{NumberTypeNode, NumberValueNode, U32};

    use super::*;

    #[test]
    fn new() {
        let node = ConstantDiscriminatorNode::new(
            ConstantValueNode::new(NumberTypeNode::le(U32), NumberValueNode::new(42u32)),
            0,
        );
        assert_eq!(
            node.constant,
            ConstantValueNode::new(NumberTypeNode::le(U32), NumberValueNode::new(42u32))
        );
        assert_eq!(node.offset, 0);
    }
}
