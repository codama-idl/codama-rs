use crate::{AccountValueNode, DataValueNode};
use codama_nodes_derive::{node_union, NodeConversions};

#[derive(NodeConversions)]
#[node_union]
#[non_exhaustive]
pub enum ConditionalValueCondition {
    AccountValue(AccountValueNode),
    DataValue(DataValueNode),
}
