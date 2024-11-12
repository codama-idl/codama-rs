use crate::{Node, ProgramNode};

#[derive(Debug)]
pub struct RootNode {
    pub program: ProgramNode,
    pub additional_programs: Vec<ProgramNode>,
}

impl Node for RootNode {
    const KIND: &'static str = "rootNode";
}
