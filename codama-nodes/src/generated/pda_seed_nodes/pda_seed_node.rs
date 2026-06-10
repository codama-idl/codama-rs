use crate::{ConstantPdaSeedNode, VariablePdaSeedNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum PdaSeedNode {
    Constant(ConstantPdaSeedNode),
    Variable(VariablePdaSeedNode),
}
