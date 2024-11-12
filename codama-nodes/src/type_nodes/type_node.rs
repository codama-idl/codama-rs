use codama_nodes_derive::IntoEnum;

use crate::{
    NumberTypeNode, PostOffsetTypeNode, PreOffsetTypeNode, SolAmountTypeNode, StringTypeNode,
    TypeNodeEnumTrait,
};

#[derive(Debug, IntoEnum)]
pub enum TypeNode {
    Number(NumberTypeNode),
    PostOffset(Box<PostOffsetTypeNode<TypeNode>>),
    PreOffset(Box<PreOffsetTypeNode<TypeNode>>),
    SolAmount(SolAmountTypeNode),
    String(StringTypeNode),
}

impl TypeNodeEnumTrait for TypeNode {}
