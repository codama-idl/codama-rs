use crate::{AccountNode, DefinedTypeNode, InstructionNode, Node, PdaNode};

#[derive(Debug)]
pub struct ProgramNode {
    // Data.
    pub name: String,
    pub version: String,

    // Children.
    pub accounts: Vec<AccountNode>,
    pub defined_types: Vec<DefinedTypeNode>,
    pub instructions: Vec<InstructionNode>,
    pub pdas: Vec<PdaNode>,
}

impl Node for ProgramNode {
    const KIND: &'static str = "programNode";
}
