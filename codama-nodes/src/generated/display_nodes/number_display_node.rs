use crate::{AmountNumberDisplayNode, DateTimeNumberDisplayNode, DurationNumberDisplayNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum NumberDisplayNode {
    Amount(AmountNumberDisplayNode),
    DateTime(DateTimeNumberDisplayNode),
    Duration(DurationNumberDisplayNode),
}
