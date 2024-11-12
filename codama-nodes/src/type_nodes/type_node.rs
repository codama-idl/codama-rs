use crate::{
    NumberTypeNode, PostOffsetTypeNode, PreOffsetTypeNode, SolAmountTypeNode, StringTypeNode,
    TypeNodeEnumTrait,
};

#[derive(Debug)]
pub enum TypeNode {
    Number(NumberTypeNode),
    PostOffset(Box<PostOffsetTypeNode<TypeNode>>),
    PreOffset(Box<PreOffsetTypeNode<TypeNode>>),
    SolAmount(SolAmountTypeNode),
    String(StringTypeNode),
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
        TypeNode::PostOffset(node.into())
    }
}

impl From<PreOffsetTypeNode<TypeNode>> for TypeNode {
    fn from(node: PreOffsetTypeNode<TypeNode>) -> Self {
        TypeNode::PreOffset(node.into())
    }
}
