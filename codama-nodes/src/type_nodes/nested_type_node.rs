use codama_nodes_derive::IntoEnum;

use crate::{
    FixedSizeTypeNode, NestedTypeNodeTrait, PostOffsetTypeNode, PreOffsetTypeNode,
    TypeNodeEnumTrait, TypeNodeTrait,
};

#[derive(IntoEnum, Debug, PartialEq)]
pub enum NestedTypeNode<T: TypeNodeTrait> {
    FixedSize(Box<FixedSizeTypeNode<NestedTypeNode<T>>>),
    PostOffset(Box<PostOffsetTypeNode<NestedTypeNode<T>>>),
    PreOffset(Box<PreOffsetTypeNode<NestedTypeNode<T>>>),
    Value(T),
    // Sentinel(Box<SentinelTypeNode<NestedTypeNode<T>>>),
    // ...
}

impl<T: TypeNodeTrait> TypeNodeEnumTrait for NestedTypeNode<T> {}

impl<T: TypeNodeTrait> NestedTypeNodeTrait<T> for NestedTypeNode<T> {
    fn get_nested_type_node(&self) -> &T {
        match self {
            NestedTypeNode::FixedSize(node) => node.get_nested_type_node(),
            NestedTypeNode::PostOffset(node) => node.get_nested_type_node(),
            NestedTypeNode::PreOffset(node) => node.get_nested_type_node(),
            NestedTypeNode::Value(value) => value,
        }
    }
}
