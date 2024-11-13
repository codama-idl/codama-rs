use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
pub struct InstructionNode {
    // Data.
    pub name: String,
}
