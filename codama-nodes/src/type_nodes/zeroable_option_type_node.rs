use crate::{ConstantValueNode, TypeNode};
use codama_nodes_derive::{Node, TypeNode};

#[derive(Node, TypeNode, Debug, PartialEq, Clone)]
pub struct ZeroableOptionTypeNode {
    // Children.
    pub item: TypeNode,
    pub zero_value: Option<ConstantValueNode>,
}

impl ZeroableOptionTypeNode {
    pub fn new<T>(item: T) -> Self
    where
        T: Into<TypeNode>,
    {
        Self {
            item: item.into(),
            zero_value: None,
        }
    }

    pub fn custom<T>(item: T, zero_value: ConstantValueNode) -> Self
    where
        T: Into<TypeNode>,
    {
        Self {
            item: item.into(),
            zero_value: Some(zero_value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Base16, NumberTypeNode, U64};

    #[test]
    fn new() {
        let node = ZeroableOptionTypeNode::new(NumberTypeNode::le(U64));
        assert_eq!(node.item, TypeNode::Number(NumberTypeNode::le(U64)));
        assert_eq!(node.zero_value, None);
    }

    #[test]
    fn custom() {
        let node = ZeroableOptionTypeNode::custom(
            NumberTypeNode::le(U64),
            ConstantValueNode::bytes(Base16, "ffffffffffffffff"),
        );
        assert_eq!(node.item, TypeNode::Number(NumberTypeNode::le(U64)));
        assert_eq!(
            node.zero_value,
            Some(ConstantValueNode::bytes(Base16, "ffffffffffffffff"))
        );
    }
}
