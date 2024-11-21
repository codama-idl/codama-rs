use crate::{ConstantPdaSeedNode, VariablePdaSeedNode};
use codama_nodes_derive::IntoEnum;
use serde::{Deserialize, Serialize};

#[derive(IntoEnum, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PdaSeedNode {
    Constant(ConstantPdaSeedNode),
    Variable(VariablePdaSeedNode),
}
