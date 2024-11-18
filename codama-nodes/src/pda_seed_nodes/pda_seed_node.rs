use crate::{ConstantPdaSeedNode, VariablePdaSeedNode};
use codama_nodes_derive::IntoEnum;

#[derive(IntoEnum, Debug, PartialEq)]
pub enum PdaSeedNode {
    Constant(ConstantPdaSeedNode),
    Variable(VariablePdaSeedNode),
}
