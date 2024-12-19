use crate::{
    CountNode, FixedCountNode, NestedTypeNode, NumberTypeNode, PrefixedCountNode,
    RemainderCountNode, TypeNode,
};
use codama_nodes_derive::type_node;

#[type_node]
pub struct SetTypeNode {
    // Children.
    pub item: Box<TypeNode>,
    pub count: CountNode,
}

impl From<SetTypeNode> for crate::Node {
    fn from(val: SetTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}

impl SetTypeNode {
    pub fn new<T, U>(item: T, count: U) -> Self
    where
        T: Into<TypeNode>,
        U: Into<CountNode>,
    {
        Self {
            item: Box::new(item.into()),
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
        let node = SetTypeNode::new(NumberTypeNode::le(U64), FixedCountNode::new(42));
        assert_eq!(*node.item, TypeNode::Number(NumberTypeNode::le(U64)));
        assert_eq!(node.count, CountNode::Fixed(FixedCountNode::new(42)));
    }

    #[test]
    fn prefixed() {
        let node = SetTypeNode::prefixed(StringTypeNode::utf8(), NumberTypeNode::le(U32));
        assert_eq!(*node.item, TypeNode::String(StringTypeNode::utf8()));
        assert_eq!(
            node.count,
            CountNode::Prefixed(PrefixedCountNode::new(NumberTypeNode::le(U32)))
        );
    }

    #[test]
    fn fixed() {
        let node = SetTypeNode::fixed(StringTypeNode::utf8(), 42);
        assert_eq!(*node.item, TypeNode::String(StringTypeNode::utf8()));
        assert_eq!(node.count, CountNode::Fixed(FixedCountNode::new(42)));
    }

    #[test]
    fn remainder() {
        let node = SetTypeNode::remainder(NumberTypeNode::le(U64));
        assert_eq!(*node.item, TypeNode::Number(NumberTypeNode::le(U64)));
        assert_eq!(node.count, CountNode::Remainder(RemainderCountNode::new()));
    }

    #[test]
    fn to_json() {
        let node = SetTypeNode::fixed(NumberTypeNode::le(U64), 42);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"setTypeNode","item":{"kind":"numberTypeNode","format":"u64","endian":"le"},"count":{"kind":"fixedCountNode","value":42}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"setTypeNode","item":{"kind":"numberTypeNode","format":"u64","endian":"le"},"count":{"kind":"fixedCountNode","value":42}}"#;
        let node: SetTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, SetTypeNode::fixed(NumberTypeNode::le(U64), 42));
    }
}
