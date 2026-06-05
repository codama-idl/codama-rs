use crate::{AccountValueNode, ArgumentValueNode, CamelCaseString, HasName};
use codama_nodes_derive::node_union;

#[node_union]
pub enum ResolverDependency {
    Account(AccountValueNode),
    Argument(ArgumentValueNode),
}

impl HasName for ResolverDependency {
    fn name(&self) -> &CamelCaseString {
        match self {
            ResolverDependency::Account(node) => node.name(),
            ResolverDependency::Argument(node) => node.name(),
        }
    }
}
