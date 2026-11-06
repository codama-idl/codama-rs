use crate::{CamelCaseString, Docs, HasIdentifier, PdaSeedNode, PluginNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct PdaNode {
    // Data.
    pub identifier: CamelCaseString,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub program_id: Option<String>,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub seeds: Vec<PdaSeedNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl HasIdentifier for PdaNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
