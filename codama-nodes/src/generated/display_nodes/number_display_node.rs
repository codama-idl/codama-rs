use crate::{AmountNumberDisplayNode, UnitNumberDisplayNode};
use codama_nodes_derive::{node_union, NodeConversions};

#[derive(NodeConversions)]
#[node_union]
pub enum NumberDisplayNode {
    Amount(AmountNumberDisplayNode),
    Unit(UnitNumberDisplayNode),
}
