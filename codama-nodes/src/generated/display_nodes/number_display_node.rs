use crate::{AmountNumberDisplayNode, UnitNumberDisplayNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum NumberDisplayNode {
    Amount(AmountNumberDisplayNode),
    Unit(UnitNumberDisplayNode),
}
