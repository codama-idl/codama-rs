use crate::{InstructionByteDeltaValue, PluginNode};
use codama_nodes_derive::node;

#[node]
pub struct InstructionByteDeltaNode {
    pub with_header: bool,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub subtract: Option<bool>,
    pub value: Box<InstructionByteDeltaValue>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}
