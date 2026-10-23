use crate::{AccountValueNode, ArgumentValueNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum PdaValueProgramId {
    Account(AccountValueNode),
    Argument(ArgumentValueNode),
}
