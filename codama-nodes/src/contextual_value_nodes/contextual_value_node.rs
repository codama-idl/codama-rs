use crate::{
    AccountBumpValueNode, AccountValueNode, ArgumentValueNode, ConditionalValueNode,
    IdentityValueNode, PayerValueNode, PdaSeedValueNode, PdaValueNode, ProgramIdValueNode,
    ResolverValueNode,
};
use codama_nodes_derive::{node_union, RegisteredNodes};

#[derive(RegisteredNodes)]
#[node_union]
pub enum RegisteredContextualValueNode {
    Account(AccountValueNode),
    AccountBump(AccountBumpValueNode),
    Argument(ArgumentValueNode),
    Conditional(Box<ConditionalValueNode>),
    Identity(IdentityValueNode),
    Payer(PayerValueNode),
    Pda(PdaValueNode),
    ProgramId(ProgramIdValueNode),
    Resolver(ResolverValueNode),

    #[registered]
    PdaSeed(PdaSeedValueNode),
}
