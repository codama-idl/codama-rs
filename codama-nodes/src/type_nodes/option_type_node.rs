use crate::{NestedTypeNode, NumberTypeNode, TypeNode, U8};
use codama_nodes_derive::type_node;

#[type_node]
pub struct OptionTypeNode {
    // Data.
    #[serde(default)]
    pub fixed: bool,

    // Children.
    pub item: Box<TypeNode>,
    pub prefix: NestedTypeNode<NumberTypeNode>,
}

impl From<OptionTypeNode> for crate::Node {
    fn from(val: OptionTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}

impl OptionTypeNode {
    pub fn new<T>(item: T) -> Self
    where
        T: Into<TypeNode>,
    {
        Self {
            fixed: false,
            item: Box::new(item.into()),
            prefix: NumberTypeNode::le(U8).into(),
        }
    }

    pub fn fixed<T>(item: T) -> Self
    where
        T: Into<TypeNode>,
    {
        Self {
            fixed: true,
            item: Box::new(item.into()),
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
        assert_eq!(*node.item, TypeNode::Number(NumberTypeNode::le(U64)));
        assert_eq!(node.prefix, NestedTypeNode::Value(NumberTypeNode::le(U8)));
        assert!(!node.fixed);
    }

    #[test]
    fn fixed() {
        let node = OptionTypeNode::fixed(NumberTypeNode::le(U64));
        assert_eq!(*node.item, TypeNode::Number(NumberTypeNode::le(U64)));
        assert_eq!(node.prefix, NestedTypeNode::Value(NumberTypeNode::le(U8)));
        assert!(node.fixed);
    }

    #[test]
    fn direct_instantiation() {
        let node = OptionTypeNode {
            fixed: true,
            item: Box::new(StringTypeNode::utf8().into()),
            prefix: NumberTypeNode::le(U64).into(),
        };

        assert_eq!(*node.item, TypeNode::String(StringTypeNode::utf8()));
        assert_eq!(node.prefix, NestedTypeNode::Value(NumberTypeNode::le(U64)));
        assert!(node.fixed);
    }

    #[test]
    fn to_json() {
        let node = OptionTypeNode::new(NumberTypeNode::le(U64));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"optionTypeNode","item":{"kind":"numberTypeNode","format":"u64","endian":"le"},"prefix":{"kind":"numberTypeNode","format":"u8","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"optionTypeNode","item":{"kind":"numberTypeNode","format":"u64","endian":"le"},"prefix":{"kind":"numberTypeNode","format":"u8","endian":"le"}}"#;
        let node: OptionTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, OptionTypeNode::new(NumberTypeNode::le(U64)));
    }

    #[test]
    fn to_json_fixed() {
        let node = OptionTypeNode::fixed(NumberTypeNode::le(U64));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"optionTypeNode","fixed":true,"item":{"kind":"numberTypeNode","format":"u64","endian":"le"},"prefix":{"kind":"numberTypeNode","format":"u8","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json_fixed() {
        let json = r#"{"kind":"optionTypeNode","fixed":true,"item":{"kind":"numberTypeNode","format":"u64","endian":"le"},"prefix":{"kind":"numberTypeNode","format":"u8","endian":"le"}}"#;
        let node: OptionTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, OptionTypeNode::fixed(NumberTypeNode::le(U64)));
    }
}
