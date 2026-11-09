use crate::{PluginNode, Text};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct InstructionDisplayNode {
    #[serde(skip_serializing_if = "crate::is_default")]
    pub intent: Option<Text>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub interpolated_intent: Option<Text>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}
