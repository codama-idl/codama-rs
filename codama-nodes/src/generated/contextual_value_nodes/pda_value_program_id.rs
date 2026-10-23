use crate::{AccountValueNode, DataValueNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum PdaValueProgramId {
    Account(AccountValueNode),
    Data(DataValueNode),
}
