use codama_nodes_derive::IntoEnum;

use super::{EnumEmptyVariantTypeNode, EnumStructVariantTypeNode, EnumTupleVariantTypeNode};

#[derive(IntoEnum, Debug, PartialEq)]
pub enum EnumVariantTypeNode {
    Empty(EnumEmptyVariantTypeNode),
    Struct(EnumStructVariantTypeNode),
    Tuple(EnumTupleVariantTypeNode),
}
