use crate::program_node::ProgramNode;

#[derive(Debug)]
pub struct RootNode {
    pub program: ProgramNode,
    pub additional_programs: Vec<ProgramNode>,
}
