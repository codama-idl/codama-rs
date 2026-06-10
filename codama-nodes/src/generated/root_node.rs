use crate::ProgramNode;
use codama_nodes_derive::node;

#[node]
pub struct RootNode {
    // Data.
    pub standard: String,
    pub version: String,

    // Children.
    pub program: ProgramNode,
    pub additional_programs: Vec<ProgramNode>,
}
