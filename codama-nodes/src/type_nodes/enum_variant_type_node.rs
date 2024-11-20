use crate::{EnumEmptyVariantTypeNode, EnumStructVariantTypeNode, EnumTupleVariantTypeNode};
use codama_nodes_derive::IntoEnum;

#[derive(IntoEnum, Debug, PartialEq, Clone)]
pub enum EnumVariantTypeNode {
    Empty(EnumEmptyVariantTypeNode),
    Struct(EnumStructVariantTypeNode),
    Tuple(EnumTupleVariantTypeNode),
}
