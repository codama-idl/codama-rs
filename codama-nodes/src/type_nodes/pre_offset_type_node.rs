use crate::{NestedTypeNodeTrait, TypeNodeEnumTrait, TypeNodeTrait};
use codama_nodes_derive::type_node;
use serde::{Deserialize, Serialize};

#[type_node]
pub struct PreOffsetTypeNode<T: TypeNodeEnumTrait> {
    // Data.
    pub offset: i32,
    pub strategy: PreOffsetStrategy,

    // Children.
    #[serde(bound = "T: TypeNodeEnumTrait")]
    pub r#type: T,
}

impl Into<crate::Node> for PreOffsetTypeNode<crate::TypeNode> {
    fn into(self) -> crate::Node {
        crate::Node::Type(self.into())
    }
}

impl<T: TypeNodeEnumTrait> PreOffsetTypeNode<T> {
    pub fn new<U>(r#type: U, strategy: PreOffsetStrategy, offset: i32) -> Self
    where
        U: Into<T>,
    {
        Self {
            r#type: r#type.into(),
            strategy,
            offset,
        }
    }

    pub fn absolute<U>(r#type: U, offset: i32) -> Self
    where
        U: Into<T>,
    {
        Self::new(r#type, PreOffsetStrategy::Absolute, offset)
    }

    pub fn padded<U>(r#type: U, offset: i32) -> Self
    where
        U: Into<T>,
    {
        Self::new(r#type, PreOffsetStrategy::Padded, offset)
    }

    pub fn relative<U>(r#type: U, offset: i32) -> Self
    where
        U: Into<T>,
    {
        Self::new(r#type, PreOffsetStrategy::Relative, offset)
    }
}

impl<T: TypeNodeEnumTrait, U: TypeNodeTrait> NestedTypeNodeTrait<U> for PreOffsetTypeNode<T>
where
    T: NestedTypeNodeTrait<U>,
{
    fn get_nested_type_node(&self) -> &U {
        self.r#type.get_nested_type_node()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PreOffsetStrategy {
    Absolute,
    Padded,
    Relative,
}

#[cfg(test)]
mod tests {
    use crate::{NestedTypeNode, NumberTypeNode, StringTypeNode, TypeNode, Utf8, U64};

    use super::*;

    #[test]
    fn new_type_node() {
        let node = PreOffsetTypeNode::<TypeNode>::new(
            StringTypeNode::new(Utf8),
            PreOffsetStrategy::Absolute,
            -42,
        );
        assert_eq!(node.r#type, TypeNode::String(StringTypeNode::new(Utf8)));
        assert_eq!(node.strategy, PreOffsetStrategy::Absolute);
        assert_eq!(node.offset, -42);
    }

    #[test]
    fn new_nested_type_node() {
        let node = PreOffsetTypeNode::<NestedTypeNode<StringTypeNode>>::new(
            StringTypeNode::new(Utf8),
            PreOffsetStrategy::Absolute,
            -42,
        );
        assert_eq!(
            node.r#type,
            NestedTypeNode::Value(StringTypeNode::new(Utf8))
        );
        assert_eq!(node.get_nested_type_node(), &StringTypeNode::new(Utf8));
        assert_eq!(node.strategy, PreOffsetStrategy::Absolute);
        assert_eq!(node.offset, -42);
    }

    #[test]
    fn absolute() {
        let node = PreOffsetTypeNode::<TypeNode>::absolute(StringTypeNode::new(Utf8), 0);
        assert_eq!(node.r#type, TypeNode::String(StringTypeNode::new(Utf8)));
        assert_eq!(node.strategy, PreOffsetStrategy::Absolute);
        assert_eq!(node.offset, 0);
    }

    #[test]
    fn relative() {
        let node = PreOffsetTypeNode::<TypeNode>::relative(StringTypeNode::new(Utf8), -4);
        assert_eq!(node.r#type, TypeNode::String(StringTypeNode::new(Utf8)));
        assert_eq!(node.strategy, PreOffsetStrategy::Relative);
        assert_eq!(node.offset, -4);
    }

    #[test]
    fn padded() {
        let node = PreOffsetTypeNode::<TypeNode>::padded(StringTypeNode::new(Utf8), 8);
        assert_eq!(node.r#type, TypeNode::String(StringTypeNode::new(Utf8)));
        assert_eq!(node.strategy, PreOffsetStrategy::Padded);
        assert_eq!(node.offset, 8);
    }

    #[test]
    fn to_json() {
        let node = PreOffsetTypeNode::<TypeNode>::padded(NumberTypeNode::le(U64), 4);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"preOffsetTypeNode","offset":4,"strategy":"padded","type":{"kind":"numberTypeNode","format":"u64","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"preOffsetTypeNode","offset":4,"strategy":"padded","type":{"kind":"numberTypeNode","format":"u64","endian":"le"}}"#;
        let node: PreOffsetTypeNode<TypeNode> = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            PreOffsetTypeNode::<TypeNode>::padded(NumberTypeNode::le(U64), 4)
        );
    }
}
