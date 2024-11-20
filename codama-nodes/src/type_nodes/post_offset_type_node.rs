use crate::{NestedTypeNodeTrait, TypeNodeEnumTrait, TypeNodeTrait};
use codama_nodes_derive::{node, TypeNode};
use serde::{Deserialize, Serialize};

#[node]
#[derive(TypeNode)]
pub struct PostOffsetTypeNode<T: TypeNodeEnumTrait> {
    // Data.
    pub offset: i32,
    pub strategy: PostOffsetStrategy,

    // Children.
    pub r#type: T,
}

impl<T: TypeNodeEnumTrait> PostOffsetTypeNode<T> {
    pub fn new<U>(r#type: U, strategy: PostOffsetStrategy, offset: i32) -> Self
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
        Self::new(r#type, PostOffsetStrategy::Absolute, offset)
    }

    pub fn padded<U>(r#type: U, offset: i32) -> Self
    where
        U: Into<T>,
    {
        Self::new(r#type, PostOffsetStrategy::Padded, offset)
    }

    pub fn pre_offset<U>(r#type: U, offset: i32) -> Self
    where
        U: Into<T>,
    {
        Self::new(r#type, PostOffsetStrategy::PreOffset, offset)
    }

    pub fn relative<U>(r#type: U, offset: i32) -> Self
    where
        U: Into<T>,
    {
        Self::new(r#type, PostOffsetStrategy::Relative, offset)
    }
}

impl<T: TypeNodeEnumTrait, U: TypeNodeTrait> NestedTypeNodeTrait<U> for PostOffsetTypeNode<T>
where
    T: NestedTypeNodeTrait<U>,
{
    fn get_nested_type_node(&self) -> &U {
        self.r#type.get_nested_type_node()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PostOffsetStrategy {
    Absolute,
    Padded,
    PreOffset,
    Relative,
}

#[cfg(test)]
mod tests {
    use crate::{NestedTypeNode, NumberTypeNode, StringTypeNode, TypeNode, Utf8, U64};

    use super::*;

    #[test]
    fn new_type_node() {
        let node = PostOffsetTypeNode::<TypeNode>::new(
            StringTypeNode::new(Utf8),
            PostOffsetStrategy::Absolute,
            -42,
        );
        assert_eq!(node.r#type, TypeNode::String(StringTypeNode::new(Utf8)));
        assert_eq!(node.strategy, PostOffsetStrategy::Absolute);
        assert_eq!(node.offset, -42);
    }

    #[test]
    fn new_nested_type_node() {
        let node = PostOffsetTypeNode::<NestedTypeNode<StringTypeNode>>::new(
            StringTypeNode::new(Utf8),
            PostOffsetStrategy::Absolute,
            -42,
        );
        assert_eq!(
            node.r#type,
            NestedTypeNode::Value(StringTypeNode::new(Utf8))
        );
        assert_eq!(node.get_nested_type_node(), &StringTypeNode::new(Utf8));
        assert_eq!(node.strategy, PostOffsetStrategy::Absolute);
        assert_eq!(node.offset, -42);
    }

    #[test]
    fn absolute() {
        let node = PostOffsetTypeNode::<TypeNode>::absolute(StringTypeNode::new(Utf8), 0);
        assert_eq!(node.r#type, TypeNode::String(StringTypeNode::new(Utf8)));
        assert_eq!(node.strategy, PostOffsetStrategy::Absolute);
        assert_eq!(node.offset, 0);
    }

    #[test]
    fn relative() {
        let node = PostOffsetTypeNode::<TypeNode>::relative(StringTypeNode::new(Utf8), -4);
        assert_eq!(node.r#type, TypeNode::String(StringTypeNode::new(Utf8)));
        assert_eq!(node.strategy, PostOffsetStrategy::Relative);
        assert_eq!(node.offset, -4);
    }

    #[test]
    fn pre_offset() {
        let node = PostOffsetTypeNode::<TypeNode>::pre_offset(StringTypeNode::new(Utf8), 0);
        assert_eq!(node.r#type, TypeNode::String(StringTypeNode::new(Utf8)));
        assert_eq!(node.strategy, PostOffsetStrategy::PreOffset);
        assert_eq!(node.offset, 0);
    }

    #[test]
    fn padded() {
        let node = PostOffsetTypeNode::<TypeNode>::padded(StringTypeNode::new(Utf8), 8);
        assert_eq!(node.r#type, TypeNode::String(StringTypeNode::new(Utf8)));
        assert_eq!(node.strategy, PostOffsetStrategy::Padded);
        assert_eq!(node.offset, 8);
    }

    #[test]
    fn to_json() {
        let node = PostOffsetTypeNode::<NestedTypeNode<NumberTypeNode>>::padded(
            NumberTypeNode::le(U64),
            4,
        );
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"postOffsetTypeNode","offset":4,"strategy":"padded","type":{"kind":"numberTypeNode","format":"u64","endian":"le"}}"#
        );
    }
}
