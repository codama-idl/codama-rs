use crate::{FixedCountNode, PrefixedCountNode, RemainderCountNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum CountNode {
    Fixed(FixedCountNode),
    Prefixed(PrefixedCountNode),
    Remainder(RemainderCountNode),
}
