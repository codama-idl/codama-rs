use crate::{
    AccountBumpValueNode, AccountDataValueNode, AccountValueNode, ConditionalValueNode,
    DataValueNode, HasKind, IdentityValueNode, PayerValueNode, PdaSeedValueNode, PdaValueNode,
    ProgramIdValueNode,
};
use codama_nodes_derive::{node_union, NodeConversions, RegisteredNodes};

#[derive(RegisteredNodes)]
#[node_union]
#[derive(NodeConversions)]
#[non_exhaustive]
pub enum RegisteredContextualValueNode {
    AccountBumpValue(AccountBumpValueNode),
    AccountDataValue(AccountDataValueNode),
    AccountValue(AccountValueNode),
    ConditionalValue(ConditionalValueNode),
    DataValue(DataValueNode),
    IdentityValue(IdentityValueNode),
    PayerValue(PayerValueNode),
    PdaValue(PdaValueNode),
    ProgramIdValue(ProgramIdValueNode),

    #[registered]
    PdaSeedValue(PdaSeedValueNode),
}
