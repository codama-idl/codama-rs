use crate::ProgramNode;
use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
pub struct RootNode {
    // Children.
    pub program: ProgramNode,
    pub additional_programs: Vec<ProgramNode>,
}
