use codama_nodes_derive::IntoEnum;

use super::{FixedCountNode, PrefixedCountNode, RemainderCountNode};

#[derive(IntoEnum, Debug, PartialEq)]
pub enum CountNode {
    Fixed(FixedCountNode),
    Prefixed(PrefixedCountNode),
    Remainder(RemainderCountNode),
}
