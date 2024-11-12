use super::{
    NumberTypeNode, NumberTypeNodeFlag, PostOffsetTypeNode, SolAmountTypeNode, StringTypeNode,
};

pub trait TypeNodeFlag {}

impl TypeNodeFlag for TypeNode {}
impl TypeNodeFlag for NumberTypeNode {}
impl TypeNodeFlag for StringTypeNode {}
impl<T: NumberTypeNodeFlag> TypeNodeFlag for SolAmountTypeNode<T> {}
impl<T: TypeNodeFlag> TypeNodeFlag for PostOffsetTypeNode<T> {}

#[derive(Debug)]
pub enum TypeNode {
    Number(NumberTypeNode),
    String(StringTypeNode),
    PostOffset(Box<PostOffsetTypeNode<TypeNode>>),
    SolAmount(SolAmountTypeNode<NestedTypeNode<NumberTypeNode>>),
}

#[derive(Debug)]
pub enum NestedTypeNode<T: TypeNodeFlag> {
    Value(T),
    PostOffset(Box<PostOffsetTypeNode<T>>),
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

impl From<NumberTypeNode> for TypeNode {
    fn from(node: NumberTypeNode) -> Self {
        TypeNode::Number(node)
    }
}

impl From<StringTypeNode> for TypeNode {
    fn from(node: StringTypeNode) -> Self {
        TypeNode::String(node)
    }
}

impl From<PostOffsetTypeNode<TypeNode>> for TypeNode {
    fn from(node: PostOffsetTypeNode<TypeNode>) -> Self {
        TypeNode::PostOffset(Box::new(node))
    }
}
