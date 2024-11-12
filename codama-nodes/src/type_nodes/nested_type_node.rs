use super::{
    NumberTypeNode, NumberTypeNodeFlag, PostOffsetTypeNode, TypeNodeEnumFlag, TypeNodeFlag,
};

impl<T: TypeNodeFlag> TypeNodeEnumFlag for NestedTypeNode<T> {}

#[derive(Debug)]
pub enum NestedTypeNode<T: TypeNodeFlag> {
    Value(T),
    PostOffset(Box<PostOffsetTypeNode<NestedTypeNode<T>>>),
    // PreOffset(Box<PreOffsetTypeNode<T>>),
    // Sentinel(Box<SentinelTypeNode<T>>),
    // ...
}

impl<T: TypeNodeFlag> NumberTypeNodeFlag for NestedTypeNode<T>
where
    T: NumberTypeNodeFlag,
{
    fn get_number_type_node(&self) -> &NumberTypeNode {
        match self {
            NestedTypeNode::Value(value) => value.get_number_type_node(),
            NestedTypeNode::PostOffset(node) => node.get_number_type_node(),
        }
    }
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
