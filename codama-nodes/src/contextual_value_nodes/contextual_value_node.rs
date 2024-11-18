use super::{AccountBumpValueNode, AccountValueNode, ArgumentValueNode};
use codama_nodes_derive::IntoEnum;

#[derive(IntoEnum, Debug, PartialEq)]
pub enum ContextualValueNode {
    Account(AccountValueNode),
    AccountBump(AccountBumpValueNode),
    Argument(ArgumentValueNode),
    // Conditional(ConditionalValueNode),
    // Identity(IdentityValueNode),
    // Payer(PayerValueNode),
    // Pda(PdaValueNode),
    // ProgramId(ProgramIdValueNode),
    // Resolver(ResolverValueNode),
}
