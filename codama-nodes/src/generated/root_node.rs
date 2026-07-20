use crate::ProgramNode;
use codama_nodes_derive::node;

#[node]
pub struct RootNode {
    // Data.
    pub standard: String,
    pub version: String,

    // Children.
    pub program: ProgramNode,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub additional_programs: Vec<ProgramNode>,
}
