use crate::{AccountValueNode, ArgumentValueNode, ResolverValueNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum ConditionalValueCondition {
    Account(AccountValueNode),
    Argument(ArgumentValueNode),
    Resolver(ResolverValueNode),
}
