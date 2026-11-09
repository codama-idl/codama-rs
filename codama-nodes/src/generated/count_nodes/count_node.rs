use crate::{FixedCountNode, PrefixedCountNode, RemainderCountNode, SentinelCountNode};
use codama_nodes_derive::{node_union, NodeConversions};

#[derive(NodeConversions)]
#[node_union]
pub enum CountNode {
    Fixed(FixedCountNode),
    Prefixed(PrefixedCountNode),
    Remainder(RemainderCountNode),
    Sentinel(SentinelCountNode),
}
