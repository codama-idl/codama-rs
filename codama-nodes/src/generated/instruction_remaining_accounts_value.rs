use crate::{ArgumentValueNode, CamelCaseString, HasName, ResolverValueNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum InstructionRemainingAccountsValue {
    Argument(ArgumentValueNode),
    Resolver(ResolverValueNode),
}

impl HasName for InstructionRemainingAccountsValue {
    fn name(&self) -> &CamelCaseString {
        match self {
            InstructionRemainingAccountsValue::Argument(node) => node.name(),
            InstructionRemainingAccountsValue::Resolver(node) => node.name(),
        }
    }
}
