use crate::NodeTrait;

#[derive(Debug)]
pub struct InstructionNode {
    // Data.
    pub name: String,
}

impl NodeTrait for InstructionNode {
    const KIND: &'static str = "instructionNode";
}
