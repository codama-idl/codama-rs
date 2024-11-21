use crate::{NestedTypeNode, NumberTypeNode};
use codama_nodes_derive::node;

#[node]
pub struct PrefixedCountNode {
    // Data.
    pub prefix: NestedTypeNode<NumberTypeNode>,
}

impl PrefixedCountNode {
    pub fn new<T>(prefix: T) -> Self
    where
        T: Into<NestedTypeNode<NumberTypeNode>>,
    {
        Self {
            prefix: prefix.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Endian, NestedTypeNodeTrait, PreOffsetTypeNode, U32};

    #[test]
    fn new() {
        let node = PrefixedCountNode::new(NumberTypeNode::le(U32));
        assert_eq!(
            node.prefix,
            NestedTypeNode::Value(NumberTypeNode::new(U32, Endian::Little))
        );
        assert_eq!(
            node.prefix.get_nested_type_node(),
            &NumberTypeNode::new(U32, Endian::Little)
        );
    }

    #[test]
    fn new_with_nested_prefix() {
        let node = PrefixedCountNode::new(PreOffsetTypeNode::absolute(NumberTypeNode::le(U32), 0));
        assert_eq!(
            node.prefix,
            NestedTypeNode::PreOffset(Box::new(PreOffsetTypeNode::absolute(
                NestedTypeNode::Value(NumberTypeNode::new(U32, Endian::Little)),
                0
            )))
        );
        assert_eq!(
            node.prefix.get_nested_type_node(),
            &NumberTypeNode::new(U32, Endian::Little)
        );
    }
}
