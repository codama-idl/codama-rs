use crate::{InstructionLifecycle, PluginNode, Text};
use codama_nodes_derive::node;

#[node]
pub struct InstructionStatusNode {
    pub lifecycle: InstructionLifecycle,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub message: Option<Text>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}
