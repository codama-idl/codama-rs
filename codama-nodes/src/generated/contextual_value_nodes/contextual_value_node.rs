use crate::{
    AccountBumpValueNode, AccountDataValueNode, AccountValueNode, ConditionalValueNode,
    DataValueNode, HasKind, IdentityValueNode, PayerValueNode, PdaSeedValueNode, PdaValueNode,
    ProgramIdValueNode,
};
use codama_nodes_derive::{node_union, RegisteredNodes};

#[derive(RegisteredNodes)]
#[node_union]
pub enum RegisteredContextualValueNode {
    Account(AccountValueNode),
    AccountBump(AccountBumpValueNode),
    AccountData(AccountDataValueNode),
    Conditional(ConditionalValueNode),
    Data(DataValueNode),
    Identity(IdentityValueNode),
    Payer(PayerValueNode),
    Pda(PdaValueNode),
    ProgramId(ProgramIdValueNode),

    #[registered]
    PdaSeed(PdaSeedValueNode),
}
