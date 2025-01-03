use crate::{EnumEmptyVariantTypeNode, EnumStructVariantTypeNode, EnumTupleVariantTypeNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum EnumVariantTypeNode {
    Empty(EnumEmptyVariantTypeNode),
    Struct(EnumStructVariantTypeNode),
    Tuple(EnumTupleVariantTypeNode),
}
