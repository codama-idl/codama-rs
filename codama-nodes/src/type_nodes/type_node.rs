use super::{NumberTypeNode, PostOffsetTypeNode, StringTypeNode};

pub trait TypeNodeFlag {}

impl TypeNodeFlag for TypeNode {}
impl TypeNodeFlag for NumberTypeNode {}
impl TypeNodeFlag for StringTypeNode {}
impl<T: TypeNodeFlag> TypeNodeFlag for PostOffsetTypeNode<T> {}

#[derive(Debug)]
pub enum TypeNode {
    Number(NumberTypeNode),
    String(StringTypeNode),
    PostOffset(Box<PostOffsetTypeNode<TypeNode>>),
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
