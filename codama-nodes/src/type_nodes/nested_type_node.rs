use super::{PostOffsetTypeNode, TypeNodeEnumFlag, TypeNodeFlag};

impl<T: TypeNodeFlag> TypeNodeEnumFlag for NestedTypeNode<T> {}

#[derive(Debug)]
pub enum NestedTypeNode<T: TypeNodeFlag> {
    Value(T),
    PostOffset(Box<PostOffsetTypeNode<NestedTypeNode<T>>>),
    // PreOffset(Box<PreOffsetTypeNode<T>>),
    // Sentinel(Box<SentinelTypeNode<T>>),
    // ...
}

impl<T: TypeNodeFlag> From<T> for NestedTypeNode<T> {
    fn from(node: T) -> Self {
        NestedTypeNode::Value(node)
    }
}

impl<T: TypeNodeFlag> From<PostOffsetTypeNode<NestedTypeNode<T>>> for NestedTypeNode<T> {
    fn from(node: PostOffsetTypeNode<NestedTypeNode<T>>) -> Self {
        NestedTypeNode::PostOffset(Box::new(node))
    }
}

pub trait NestedTypeNodeFlag<T: TypeNodeFlag> {
    fn get_nested_type_node(&self) -> &T;
}

impl<T: TypeNodeFlag> NestedTypeNodeFlag<T> for NestedTypeNode<T> {
    fn get_nested_type_node(&self) -> &T {
        match self {
            NestedTypeNode::Value(value) => value,
            NestedTypeNode::PostOffset(node) => node.get_nested_type_node(),
        }
    }
}
