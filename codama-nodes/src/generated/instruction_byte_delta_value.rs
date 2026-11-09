use crate::{AccountLinkNode, DataValueNode, IntegerValueNode};
use codama_nodes_derive::{node_union, NodeConversions};

#[derive(NodeConversions)]
#[node_union]
pub enum InstructionByteDeltaValue {
    AccountLink(AccountLinkNode),
    DataValue(DataValueNode),
    IntegerValue(IntegerValueNode),
}
