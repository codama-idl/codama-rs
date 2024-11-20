use crate::{NestedTypeNode, NumberTypeNode};
use codama_nodes_derive::{Node, TypeNode};

#[derive(Node, TypeNode, Debug, PartialEq, Clone)]
pub struct BooleanTypeNode {
    // Children.
    pub size: NestedTypeNode<NumberTypeNode>,
}

impl BooleanTypeNode {
    pub fn new<T>(size: T) -> Self
    where
        T: Into<NestedTypeNode<NumberTypeNode>>,
    {
        Self { size: size.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NestedTypeNodeTrait, NumberTypeNode, PostOffsetTypeNode, PreOffsetTypeNode, U64};

    #[test]
    fn new() {
        let node = BooleanTypeNode::new(NumberTypeNode::le(U64));
        assert_eq!(node.size, NestedTypeNode::Value(NumberTypeNode::le(U64,)));
    }

    #[test]
    fn new_with_nested_size() {
        let node = BooleanTypeNode::new(PostOffsetTypeNode::pre_offset(
            PreOffsetTypeNode::absolute(NumberTypeNode::le(U64), 0),
            0,
        ));
        assert_eq!(
            node.size,
            NestedTypeNode::PostOffset(Box::new(PostOffsetTypeNode::pre_offset(
                NestedTypeNode::PreOffset(Box::new(PreOffsetTypeNode::absolute(
                    NestedTypeNode::Value(NumberTypeNode::le(U64)),
                    0
                ))),
                0,
            )))
        );
        assert_eq!(node.size.get_nested_type_node(), &NumberTypeNode::le(U64,));
    }
}
