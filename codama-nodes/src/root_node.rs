use crate::ProgramNode;
use codama_nodes_derive::Node;

#[derive(Debug, Node)]
pub struct RootNode {
    // Children.
    pub program: ProgramNode,
    pub additional_programs: Vec<ProgramNode>,
}
