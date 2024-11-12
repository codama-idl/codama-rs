use crate::Node;

#[derive(Debug)]
pub struct InstructionNode {
    pub name: String,
}

impl Node for InstructionNode {
    const KIND: &'static str = "instructionNode";
}
