use crate::{PluginNode, ProgramNode};
use codama_nodes_derive::node;

#[node]
pub struct RootNode {
    pub standard: String,
    pub version: String,
    pub program: ProgramNode,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub additional_programs: Vec<ProgramNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}
