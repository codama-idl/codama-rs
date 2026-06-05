use crate::{AccountValueNode, ArgumentValueNode, CamelCaseString, HasName, ResolverValueNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum ConditionalValueCondition {
    Account(AccountValueNode),
    Argument(ArgumentValueNode),
    Resolver(ResolverValueNode),
}

impl HasName for ConditionalValueCondition {
    fn name(&self) -> &CamelCaseString {
        match self {
            ConditionalValueCondition::Account(node) => node.name(),
            ConditionalValueCondition::Argument(node) => node.name(),
            ConditionalValueCondition::Resolver(node) => node.name(),
        }
    }
}
