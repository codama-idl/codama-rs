use crate::{AccountValueNode, DataValueNode};
use codama_nodes_derive::{node_union, NodeConversions};

#[derive(NodeConversions)]
#[node_union]
pub enum ConditionalValueCondition {
    Account(AccountValueNode),
    Data(DataValueNode),
}
