use crate::{DisplaySkip, PluginNode, Text};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct InstructionAccountDisplayNode {
    #[serde(skip_serializing_if = "crate::is_default")]
    pub label: Option<Text>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub skip: Option<DisplaySkip>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}
