use crate::{FixedCountNode, PrefixedCountNode, RemainderCountNode};
use codama_nodes_derive::IntoEnum;

#[derive(IntoEnum, Debug, PartialEq, Clone)]
pub enum CountNode {
    Fixed(FixedCountNode),
    Prefixed(PrefixedCountNode),
    Remainder(RemainderCountNode),
}
