use crate::{
    CountNode, FixedCountNode, NestedTypeNode, NumberTypeNode, PrefixedCountNode,
    RemainderCountNode, TypeNode,
};
use codama_nodes_derive::{Node, TypeNode};

#[derive(Node, TypeNode, Debug, PartialEq)]
pub struct ArrayTypeNode {
    // Children.
    pub item: TypeNode,
    pub count: CountNode,
}

impl ArrayTypeNode {
    pub fn new<T, U>(item: T, count: U) -> Self
    where
        T: Into<TypeNode>,
        U: Into<CountNode>,
    {
        Self {
            item: item.into(),
            count: count.into(),
        }
    }

    pub fn fixed<T>(item: T, value: usize) -> Self
    where
        T: Into<TypeNode>,
    {
        Self::new(item, FixedCountNode::new(value))
    }

    pub fn prefixed<T, U>(item: T, prefix: U) -> Self
    where
        T: Into<TypeNode>,
        U: Into<NestedTypeNode<NumberTypeNode>>,
    {
        Self::new(item, PrefixedCountNode::new(prefix))
    }

    pub fn remainder<T>(item: T) -> Self
    where
        T: Into<TypeNode>,
    {
        Self::new(item, RemainderCountNode::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NumberTypeNode, StringTypeNode, U32, U64};

    #[test]
    fn new() {
        let node = ArrayTypeNode::new(NumberTypeNode::le(U64), FixedCountNode::new(42));
        assert_eq!(node.item, TypeNode::Number(NumberTypeNode::le(U64)));
        assert_eq!(node.count, CountNode::Fixed(FixedCountNode::new(42)));
    }

    #[test]
    fn prefixed() {
        let node = ArrayTypeNode::prefixed(StringTypeNode::utf8(), NumberTypeNode::le(U32));
        assert_eq!(node.item, TypeNode::String(StringTypeNode::utf8()));
        assert_eq!(
            node.count,
            CountNode::Prefixed(PrefixedCountNode::new(NumberTypeNode::le(U32)))
        );
    }

    #[test]
    fn fixed() {
        let node = ArrayTypeNode::fixed(StringTypeNode::utf8(), 42);
        assert_eq!(node.item, TypeNode::String(StringTypeNode::utf8()));
        assert_eq!(node.count, CountNode::Fixed(FixedCountNode::new(42)));
    }

    #[test]
    fn remainder() {
        let node = ArrayTypeNode::remainder(NumberTypeNode::le(U64));
        assert_eq!(node.item, TypeNode::Number(NumberTypeNode::le(U64)));
        assert_eq!(node.count, CountNode::Remainder(RemainderCountNode::new()));
    }
}
