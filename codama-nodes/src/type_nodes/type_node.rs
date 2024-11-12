use super::{
    NumberTypeNode, PostOffsetTypeNode, SolAmountTypeNode, StringTypeNode, TypeNodeEnumTrait,
};

#[derive(Debug)]
pub enum TypeNode {
    Number(NumberTypeNode),
    String(StringTypeNode),
    PostOffset(Box<PostOffsetTypeNode<TypeNode>>),
    SolAmount(SolAmountTypeNode),
}

impl TypeNodeEnumTrait for TypeNode {}

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
