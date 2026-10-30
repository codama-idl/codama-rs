use crate::{AccountValueNode, ArgumentValueNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum ResolverDependency {
    Account(AccountValueNode),
    Argument(ArgumentValueNode),
}
