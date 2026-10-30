use crate::{
    ArrayTypeNode, CountNode, FixedCountNode, IntegerTypeNode, PrefixedCountNode,
    RemainderCountNode, TypeNode,
};

impl ArrayTypeNode {
    pub fn new<T, U>(item: T, count: U) -> Self
    where
        T: Into<TypeNode>,
        U: Into<CountNode>,
    {
        Self {
            item: Box::new(item.into()),
            count: Box::new(count.into()),
            transforms: vec![],
            plugins: vec![],
        }
    }

    pub fn fixed<T>(item: T, value: u64) -> Self
    where
        T: Into<TypeNode>,
    {
        Self::new(item, FixedCountNode::new(value))
    }

    pub fn prefixed<T, U>(item: T, prefix: U) -> Self
    where
        T: Into<TypeNode>,
        U: Into<IntegerTypeNode>,
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
    use crate::{StringTypeNode, U32, U64};

    #[test]
    fn new() {
        let node = ArrayTypeNode::new(IntegerTypeNode::le(U64), FixedCountNode::new(42));
        assert_eq!(*node.item, TypeNode::Integer(IntegerTypeNode::le(U64)));
        assert_eq!(*node.count, CountNode::Fixed(FixedCountNode::new(42)));
    }

    #[test]
    fn prefixed() {
        let node = ArrayTypeNode::prefixed(StringTypeNode::utf8(), IntegerTypeNode::le(U32));
        assert_eq!(*node.item, TypeNode::String(StringTypeNode::utf8()));
        assert_eq!(
            *node.count,
            CountNode::Prefixed(PrefixedCountNode::new(IntegerTypeNode::le(U32)))
        );
    }

    #[test]
    fn fixed() {
        let node = ArrayTypeNode::fixed(StringTypeNode::utf8(), 42);
        assert_eq!(*node.item, TypeNode::String(StringTypeNode::utf8()));
        assert_eq!(*node.count, CountNode::Fixed(FixedCountNode::new(42)));
    }

    #[test]
    fn remainder() {
        let node = ArrayTypeNode::remainder(IntegerTypeNode::le(U64));
        assert_eq!(*node.item, TypeNode::Integer(IntegerTypeNode::le(U64)));
        assert_eq!(*node.count, CountNode::Remainder(RemainderCountNode::new()));
    }

    #[test]
    fn to_json() {
        let node = ArrayTypeNode::fixed(IntegerTypeNode::le(U64), 42);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"arrayTypeNode","item":{"kind":"integerTypeNode","format":"u64","endian":"le"},"count":{"kind":"fixedCountNode","value":42}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"arrayTypeNode","item":{"kind":"integerTypeNode","format":"u64","endian":"le"},"count":{"kind":"fixedCountNode","value":42}}"#;
        let node: ArrayTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, ArrayTypeNode::fixed(IntegerTypeNode::le(U64), 42));
    }
}
