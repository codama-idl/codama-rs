use crate::{ArgumentValueNode, ResolverValueNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum InstructionRemainingAccountsValue {
    Argument(ArgumentValueNode),
    Resolver(ResolverValueNode),
}
