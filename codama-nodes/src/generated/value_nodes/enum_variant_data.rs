use crate::{StructValueNode, TupleValueNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum EnumVariantData {
    Struct(StructValueNode),
    Tuple(TupleValueNode),
}
