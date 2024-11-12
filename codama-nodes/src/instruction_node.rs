use crate::Node;

#[derive(Debug)]
pub struct InstructionNode {
    // Data.
    pub name: String,
}

impl Node for InstructionNode {
    const KIND: &'static str = "instructionNode";
}
