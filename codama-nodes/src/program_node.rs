use crate::{AccountNode, DefinedTypeNode, InstructionNode, PdaNode};
use codama_nodes_derive::Node;

#[derive(Node, Debug)]
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
