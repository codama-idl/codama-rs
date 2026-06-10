use crate::{AccountLinkNode, ArgumentValueNode, NumberValueNode, ResolverValueNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum InstructionByteDeltaValue {
    AccountLink(AccountLinkNode),
    ArgumentValue(ArgumentValueNode),
    NumberValue(NumberValueNode),
    ResolverValue(ResolverValueNode),
}
