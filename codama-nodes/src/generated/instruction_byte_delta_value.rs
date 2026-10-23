use crate::{AccountLinkNode, DataValueNode, IntegerValueNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum InstructionByteDeltaValue {
    AccountLink(AccountLinkNode),
    DataValue(DataValueNode),
    IntegerValue(IntegerValueNode),
}
