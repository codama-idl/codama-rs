use super::{
    AccountBumpValueNode, AccountValueNode, ArgumentValueNode, ConditionalValueNode,
    IdentityValueNode, PayerValueNode, PdaValueNode, ProgramIdValueNode, ResolverValueNode,
};
use codama_nodes_derive::IntoEnum;

#[derive(IntoEnum, Debug, PartialEq)]
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
