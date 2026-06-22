use crate::{
    AccountBumpValueNode, AccountFieldValueNode, AccountValueNode, ArgumentValueNode,
    ConditionalValueNode, HasKind, IdentityValueNode, PayerValueNode, PdaSeedValueNode,
    PdaValueNode, ProgramIdValueNode, ResolverValueNode,
};
use codama_nodes_derive::{node_union, RegisteredNodes};

#[derive(RegisteredNodes)]
#[node_union]
pub enum RegisteredContextualValueNode {
    Account(AccountValueNode),
    AccountBump(AccountBumpValueNode),
    AccountField(AccountFieldValueNode),
    Argument(ArgumentValueNode),
    Conditional(ConditionalValueNode),
    Identity(IdentityValueNode),
    Payer(PayerValueNode),
    Pda(PdaValueNode),
    ProgramId(ProgramIdValueNode),
    Resolver(ResolverValueNode),

    #[registered]
    PdaSeed(PdaSeedValueNode),
}
