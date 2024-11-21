use crate::{EnumEmptyVariantTypeNode, EnumStructVariantTypeNode, EnumTupleVariantTypeNode};
use codama_nodes_derive::IntoEnum;
use serde::{Deserialize, Serialize};

#[derive(IntoEnum, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnumVariantTypeNode {
    Empty(EnumEmptyVariantTypeNode),
    Struct(EnumStructVariantTypeNode),
    Tuple(EnumTupleVariantTypeNode),
}
