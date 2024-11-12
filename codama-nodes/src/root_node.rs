use crate::{NodeTrait, ProgramNode};

#[derive(Debug)]
pub struct RootNode {
    // Children.
    pub program: ProgramNode,
    pub additional_programs: Vec<ProgramNode>,
}

impl NodeTrait for RootNode {
    const KIND: &'static str = "rootNode";
}
