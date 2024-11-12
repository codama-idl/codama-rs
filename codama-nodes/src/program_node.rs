use crate::{
    account_node::AccountNode, defined_type_node::DefinedTypeNode,
    instruction_node::InstructionNode, pda_node::PdaNode,
};

#[derive(Debug)]
pub struct ProgramNode {
    pub name: String,
    pub version: String,
    pub accounts: Vec<AccountNode>,
    pub instructions: Vec<InstructionNode>,
    pub defined_types: Vec<DefinedTypeNode>,
    pub pdas: Vec<PdaNode>,
}
