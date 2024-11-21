use crate::{
    AccountBumpValueNode, AccountValueNode, ArgumentValueNode, ConditionalValueNode,
    IdentityValueNode, PayerValueNode, PdaValueNode, ProgramIdValueNode, ResolverValueNode,
};
use codama_nodes_derive::node_union;

#[node_union]
pub enum ContextualValueNode {
    Account(AccountValueNode),
    AccountBump(AccountBumpValueNode),
    Argument(ArgumentValueNode),
    Conditional(Box<ConditionalValueNode>),
    Identity(IdentityValueNode),
    Payer(PayerValueNode),
    Pda(PdaValueNode),
    ProgramId(ProgramIdValueNode),
    Resolver(ResolverValueNode),
}
