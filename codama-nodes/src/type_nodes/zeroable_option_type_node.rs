use crate::{ConstantValueNode, TypeNode, ZeroableOptionTypeNode};

impl ZeroableOptionTypeNode {
    pub fn new<T>(item: T) -> Self
    where
        T: Into<TypeNode>,
    {
        Self {
            item: Box::new(item.into()),
            zero_value: None,
            transforms: vec![],
            plugins: vec![],
        }
    }

    pub fn custom<T>(item: T, zero_value: ConstantValueNode) -> Self
    where
        T: Into<TypeNode>,
    {
        Self {
            item: Box::new(item.into()),
            zero_value: Some(zero_value),
            transforms: vec![],
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Base16, IntegerTypeNode, U64};

    #[test]
    fn new() {
        let node = ZeroableOptionTypeNode::new(IntegerTypeNode::le(U64));
        assert_eq!(*node.item, TypeNode::Integer(IntegerTypeNode::le(U64)));
        assert_eq!(node.zero_value, None);
    }

    #[test]
    fn custom() {
        let node = ZeroableOptionTypeNode::custom(
            IntegerTypeNode::le(U64),
            ConstantValueNode::bytes(Base16, "ffffffffffffffff"),
        );
        assert_eq!(*node.item, TypeNode::Integer(IntegerTypeNode::le(U64)));
        assert_eq!(
            node.zero_value,
            Some(ConstantValueNode::bytes(Base16, "ffffffffffffffff"))
        );
    }

    #[test]
    fn to_json() {
        let node = ZeroableOptionTypeNode::new(IntegerTypeNode::le(U64));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"zeroableOptionTypeNode","item":{"kind":"integerTypeNode","format":"u64","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"zeroableOptionTypeNode","item":{"kind":"integerTypeNode","format":"u64","endian":"le"}}"#;
        let node: ZeroableOptionTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, ZeroableOptionTypeNode::new(IntegerTypeNode::le(U64)));
    }
}
