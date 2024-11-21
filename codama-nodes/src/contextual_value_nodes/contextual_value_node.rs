use crate::{
    AccountBumpValueNode, AccountValueNode, ArgumentValueNode, ConditionalValueNode,
    IdentityValueNode, PayerValueNode, PdaValueNode, ProgramIdValueNode, ResolverValueNode,
};
use codama_nodes_derive::IntoEnum;
use serde::{Deserialize, Serialize};

#[derive(IntoEnum, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
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
