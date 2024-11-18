use crate::{NestedTypeNode, NumberTypeNode, TypeNode, U8};
use codama_nodes_derive::{Node, TypeNode};

#[derive(Node, TypeNode, Debug, PartialEq)]
pub struct OptionTypeNode {
    // Data.
    pub fixed: bool,

    // Children.
    pub item: TypeNode,
    pub prefix: NestedTypeNode<NumberTypeNode>,
}

impl OptionTypeNode {
    pub fn new<T>(item: T) -> Self
    where
        T: Into<TypeNode>,
    {
        Self {
            fixed: false,
            item: item.into(),
            prefix: NumberTypeNode::le(U8).into(),
        }
    }

    pub fn fixed<T>(item: T) -> Self
    where
        T: Into<TypeNode>,
    {
        Self {
            fixed: true,
            item: item.into(),
            prefix: NumberTypeNode::le(U8).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NumberTypeNode, StringTypeNode, U64};

    #[test]
    fn new() {
        let node = OptionTypeNode::new(NumberTypeNode::le(U64));
        assert_eq!(node.item, TypeNode::Number(NumberTypeNode::le(U64)));
        assert_eq!(node.prefix, NestedTypeNode::Value(NumberTypeNode::le(U8)));
        assert_eq!(node.fixed, false);
    }

    #[test]
    fn fixed() {
        let node = OptionTypeNode::fixed(NumberTypeNode::le(U64));
        assert_eq!(node.item, TypeNode::Number(NumberTypeNode::le(U64)));
        assert_eq!(node.prefix, NestedTypeNode::Value(NumberTypeNode::le(U8)));
        assert_eq!(node.fixed, true);
    }

    #[test]
    fn direct_instantiation() {
        let node = OptionTypeNode {
            fixed: true,
            item: StringTypeNode::utf8().into(),
            prefix: NumberTypeNode::le(U64).into(),
        };

        assert_eq!(node.item, TypeNode::String(StringTypeNode::utf8()));
        assert_eq!(node.prefix, NestedTypeNode::Value(NumberTypeNode::le(U64)));
        assert_eq!(node.fixed, true);
    }
}
