use crate::{InstructionLifecycle, PluginNode, Text};
use codama_nodes_derive::node;

#[node]
pub struct InstructionStatusNode {
    // Data.
    pub lifecycle: InstructionLifecycle,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub message: Option<Text>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}
