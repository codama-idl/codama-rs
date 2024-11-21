use crate::{NestedTypeNode, NumberTypeNode};
use codama_nodes_derive::{node, TypeNode};

#[node]
#[derive(TypeNode)]
pub struct SolAmountTypeNode {
    // Children.
    pub number: NestedTypeNode<NumberTypeNode>,
}

impl SolAmountTypeNode {
    pub fn new<T>(number: T) -> Self
    where
        T: Into<NestedTypeNode<NumberTypeNode>>,
    {
        Self {
            number: number.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Endian, NestedTypeNodeTrait, NumberTypeNode, PostOffsetTypeNode, PreOffsetTypeNode, U64,
    };

    #[test]
    fn new() {
        let node = SolAmountTypeNode::new(NumberTypeNode::new(U64, Endian::Big));
        assert_eq!(
            node.number,
            NestedTypeNode::Value(NumberTypeNode::new(U64, Endian::Big))
        );
    }

    #[test]
    fn new_with_explicit_value() {
        let node =
            SolAmountTypeNode::new(NestedTypeNode::Value(NumberTypeNode::new(U64, Endian::Big)));
        assert_eq!(
            node.number,
            NestedTypeNode::Value(NumberTypeNode::new(U64, Endian::Big))
        );
    }

    #[test]
    fn new_with_nested_value() {
        let node = SolAmountTypeNode::new(PostOffsetTypeNode::pre_offset(
            PreOffsetTypeNode::absolute(NumberTypeNode::new(U64, Endian::Big), 0),
            0,
        ));
        assert_eq!(
            node.number,
            NestedTypeNode::PostOffset(Box::new(PostOffsetTypeNode::pre_offset(
                NestedTypeNode::PreOffset(Box::new(PreOffsetTypeNode::absolute(
                    NestedTypeNode::Value(NumberTypeNode::new(U64, Endian::Big)),
                    0
                ))),
                0,
            )))
        );
        assert_eq!(
            node.number.get_nested_type_node(),
            &NumberTypeNode::new(U64, Endian::Big)
        );
    }
}
