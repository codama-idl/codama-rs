use crate::{NestedTypeNode, NestedTypeNodeTrait, TypeNode, TypeNodeTrait, TypeNodeUnionTrait};
use codama_nodes_derive::nestable_type_node;
use serde::{Deserialize, Serialize};

#[nestable_type_node]
pub struct PreOffsetTypeNode<T: TypeNodeUnionTrait> {
    // Data.
    pub offset: i32,
    pub strategy: PreOffsetStrategy,

    // Children.
    #[serde(bound = "T: TypeNodeUnionTrait")]
    pub r#type: Box<T>,
}

impl From<PreOffsetTypeNode<crate::TypeNode>> for crate::Node {
    fn from(val: PreOffsetTypeNode<crate::TypeNode>) -> Self {
        crate::Node::Type(val.into())
    }
}

impl<T: TypeNodeUnionTrait> PreOffsetTypeNode<T> {
    pub fn new<U>(r#type: U, strategy: PreOffsetStrategy, offset: i32) -> Self
    where
        U: Into<T>,
    {
        Self {
            r#type: Box::new(r#type.into()),
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

impl<T: TypeNodeTrait> NestedTypeNodeTrait<T> for PreOffsetTypeNode<NestedTypeNode<T>> {
    type Mapped<U: TypeNodeTrait> = PreOffsetTypeNode<NestedTypeNode<U>>;

    fn get_nested_type_node(&self) -> &T {
        self.r#type.get_nested_type_node()
    }

    fn map_nested_type_node<U: TypeNodeTrait, F: FnOnce(T) -> U>(self, f: F) -> Self::Mapped<U> {
        PreOffsetTypeNode {
            r#type: Box::new(self.r#type.map_nested_type_node(f)),
            strategy: self.strategy,
            offset: self.offset,
        }
    }
}

impl<T: TypeNodeTrait> TypeNodeTrait for PreOffsetTypeNode<NestedTypeNode<T>> {
    fn into_type_node(self) -> TypeNode {
        TypeNode::PreOffset(PreOffsetTypeNode {
            offset: self.offset,
            strategy: self.strategy,
            r#type: Box::new((*self.r#type).into()),
        })
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
        assert_eq!(*node.r#type, TypeNode::String(StringTypeNode::new(Utf8)));
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
            *node.r#type,
            NestedTypeNode::Value(StringTypeNode::new(Utf8))
        );
        assert_eq!(node.get_nested_type_node(), &StringTypeNode::new(Utf8));
        assert_eq!(node.strategy, PreOffsetStrategy::Absolute);
        assert_eq!(node.offset, -42);
    }

    #[test]
    fn absolute() {
        let node = PreOffsetTypeNode::<TypeNode>::absolute(StringTypeNode::new(Utf8), 0);
        assert_eq!(*node.r#type, TypeNode::String(StringTypeNode::new(Utf8)));
        assert_eq!(node.strategy, PreOffsetStrategy::Absolute);
        assert_eq!(node.offset, 0);
    }

    #[test]
    fn relative() {
        let node = PreOffsetTypeNode::<TypeNode>::relative(StringTypeNode::new(Utf8), -4);
        assert_eq!(*node.r#type, TypeNode::String(StringTypeNode::new(Utf8)));
        assert_eq!(node.strategy, PreOffsetStrategy::Relative);
        assert_eq!(node.offset, -4);
    }

    #[test]
    fn padded() {
        let node = PreOffsetTypeNode::<TypeNode>::padded(StringTypeNode::new(Utf8), 8);
        assert_eq!(*node.r#type, TypeNode::String(StringTypeNode::new(Utf8)));
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
