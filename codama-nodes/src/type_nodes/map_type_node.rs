use crate::{
    CountNode, FixedCountNode, NestedTypeNode, NumberTypeNode, PrefixedCountNode,
    RemainderCountNode, TypeNode,
};
use codama_nodes_derive::type_node;

#[type_node]
pub struct MapTypeNode {
    // Children.
    pub key: TypeNode,
    pub value: TypeNode,
    pub count: CountNode,
}

impl Into<crate::Node> for MapTypeNode {
    fn into(self) -> crate::Node {
        crate::Node::Type(self.into())
    }
}

impl MapTypeNode {
    pub fn new<K, V, C>(key: K, value: V, count: C) -> Self
    where
        K: Into<TypeNode>,
        V: Into<TypeNode>,
        C: Into<CountNode>,
    {
        Self {
            key: key.into(),
            value: value.into(),
            count: count.into(),
        }
    }

    pub fn fixed<K, V>(key: K, value: V, size: usize) -> Self
    where
        K: Into<TypeNode>,
        V: Into<TypeNode>,
    {
        Self::new(key, value, FixedCountNode::new(size))
    }

    pub fn prefixed<K, V, P>(key: K, value: V, prefix: P) -> Self
    where
        K: Into<TypeNode>,
        V: Into<TypeNode>,
        P: Into<NestedTypeNode<NumberTypeNode>>,
    {
        Self::new(key, value, PrefixedCountNode::new(prefix))
    }

    pub fn remainder<K, V>(key: K, value: V) -> Self
    where
        K: Into<TypeNode>,
        V: Into<TypeNode>,
    {
        Self::new(key, value, RemainderCountNode::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FixedSizeTypeNode, NumberTypeNode, StringTypeNode, U32, U64};

    #[test]
    fn new() {
        let node = MapTypeNode::new(
            FixedSizeTypeNode::new(StringTypeNode::utf8(), 10),
            NumberTypeNode::le(U64),
            FixedCountNode::new(42),
        );
        assert_eq!(
            node.key,
            TypeNode::FixedSize(Box::new(FixedSizeTypeNode::new(StringTypeNode::utf8(), 10)))
        );
        assert_eq!(node.value, TypeNode::Number(NumberTypeNode::le(U64)));
        assert_eq!(node.count, CountNode::Fixed(FixedCountNode::new(42)));
    }

    #[test]
    fn prefixed() {
        let node = MapTypeNode::prefixed(
            StringTypeNode::utf8(),
            NumberTypeNode::le(U64),
            NumberTypeNode::le(U32),
        );
        assert_eq!(node.key, TypeNode::String(StringTypeNode::utf8()));
        assert_eq!(node.value, TypeNode::Number(NumberTypeNode::le(U64)));
        assert_eq!(
            node.count,
            CountNode::Prefixed(PrefixedCountNode::new(NumberTypeNode::le(U32)))
        );
    }

    #[test]
    fn fixed() {
        let node = MapTypeNode::fixed(StringTypeNode::utf8(), NumberTypeNode::le(U64), 42);
        assert_eq!(node.key, TypeNode::String(StringTypeNode::utf8()));
        assert_eq!(node.value, TypeNode::Number(NumberTypeNode::le(U64)));
        assert_eq!(node.count, CountNode::Fixed(FixedCountNode::new(42)));
    }

    #[test]
    fn remainder() {
        let node = MapTypeNode::remainder(StringTypeNode::utf8(), NumberTypeNode::le(U64));
        assert_eq!(node.key, TypeNode::String(StringTypeNode::utf8()));
        assert_eq!(node.value, TypeNode::Number(NumberTypeNode::le(U64)));
        assert_eq!(node.count, CountNode::Remainder(RemainderCountNode::new()));
    }

    #[test]
    fn to_json() {
        let node = MapTypeNode::fixed(StringTypeNode::utf8(), NumberTypeNode::le(U64), 42);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"mapTypeNode","key":{"kind":"stringTypeNode","encoding":"utf8"},"value":{"kind":"numberTypeNode","format":"u64","endian":"le"},"count":{"kind":"fixedCountNode","value":42}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"mapTypeNode","key":{"kind":"stringTypeNode","encoding":"utf8"},"value":{"kind":"numberTypeNode","format":"u64","endian":"le"},"count":{"kind":"fixedCountNode","value":42}}"#;
        let node: MapTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            MapTypeNode::fixed(StringTypeNode::utf8(), NumberTypeNode::le(U64), 42)
        );
    }
}
