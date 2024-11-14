use codama_nodes_derive::IntoEnum;

use crate::{
    NumberTypeNode, PostOffsetTypeNode, PreOffsetTypeNode, SolAmountTypeNode, StringTypeNode,
    TypeNodeEnumTrait,
};

use super::{
    AmountTypeNode, ArrayTypeNode, BooleanTypeNode, BytesTypeNode, DateTimeTypeNode,
    StructTypeNode, TupleTypeNode,
};

#[derive(IntoEnum, Debug, PartialEq)]
pub enum TypeNode {
    Amount(AmountTypeNode),
    Array(Box<ArrayTypeNode>),
    Boolean(BooleanTypeNode),
    Bytes(BytesTypeNode),
    DateTime(DateTimeTypeNode),
    Number(NumberTypeNode),
    PostOffset(Box<PostOffsetTypeNode<TypeNode>>),
    PreOffset(Box<PreOffsetTypeNode<TypeNode>>),
    SolAmount(SolAmountTypeNode),
    String(StringTypeNode),
    Struct(StructTypeNode),
    Tuple(TupleTypeNode),
}

impl TypeNodeEnumTrait for TypeNode {}
