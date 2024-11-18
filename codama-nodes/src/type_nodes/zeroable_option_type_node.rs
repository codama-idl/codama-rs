use crate::TypeNode;
use codama_nodes_derive::{Node, TypeNode};

#[derive(Node, TypeNode, Debug, PartialEq)]
pub struct ZeroableOptionTypeNode {
    // Children.
    pub item: TypeNode,
    pub zero_value: Option<()>, // TODO ConstantValueNode
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

    pub fn custom<T>(item: T, zero_value: ()) -> Self
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
    use crate::{NumberTypeNode, U64};

    #[test]
    fn new() {
        let node = ZeroableOptionTypeNode::new(NumberTypeNode::le(U64));
        assert_eq!(node.item, TypeNode::Number(NumberTypeNode::le(U64)));
        assert_eq!(node.zero_value, None);
    }

    #[test]
    fn custom() {
        let node = ZeroableOptionTypeNode::custom(NumberTypeNode::le(U64), ());
        assert_eq!(node.item, TypeNode::Number(NumberTypeNode::le(U64)));
        assert_eq!(node.zero_value, Some(())); // TODO ConstantValueNode
    }
}
