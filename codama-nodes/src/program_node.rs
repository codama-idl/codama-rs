use crate::{AccountNode, DefinedTypeNode, InstructionNode, Node, PdaNode};

#[derive(Debug)]
pub struct ProgramNode {
    pub name: String,
    pub version: String,
    pub accounts: Vec<AccountNode>,
    pub instructions: Vec<InstructionNode>,
    pub defined_types: Vec<DefinedTypeNode>,
    pub pdas: Vec<PdaNode>,
}

impl Node for ProgramNode {
    const KIND: &'static str = "programNode";
}
