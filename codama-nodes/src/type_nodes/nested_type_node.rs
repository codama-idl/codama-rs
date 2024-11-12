use crate::{NestedTypeNodeTrait, PostOffsetTypeNode, TypeNodeEnumTrait, TypeNodeTrait};

#[derive(Debug)]
pub enum NestedTypeNode<T: TypeNodeTrait> {
    Value(T),
    PostOffset(Box<PostOffsetTypeNode<NestedTypeNode<T>>>),
    // PreOffset(Box<PreOffsetTypeNode<T>>),
    // Sentinel(Box<SentinelTypeNode<T>>),
    // ...
}

impl<T: TypeNodeTrait> TypeNodeEnumTrait for NestedTypeNode<T> {}

impl<T: TypeNodeTrait> NestedTypeNodeTrait<T> for NestedTypeNode<T> {
    fn get_nested_type_node(&self) -> &T {
        match self {
            NestedTypeNode::Value(value) => value,
            NestedTypeNode::PostOffset(node) => node.get_nested_type_node(),
        }
    }
}

impl<T: TypeNodeTrait> From<T> for NestedTypeNode<T> {
    fn from(node: T) -> Self {
        NestedTypeNode::Value(node)
    }
}

impl<T: TypeNodeTrait> From<PostOffsetTypeNode<NestedTypeNode<T>>> for NestedTypeNode<T> {
    fn from(node: PostOffsetTypeNode<NestedTypeNode<T>>) -> Self {
        NestedTypeNode::PostOffset(Box::new(node))
    }
}
