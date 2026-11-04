use crate::{Docs, HasIdentifier, Identifier, PdaSeedNode, PluginNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct PdaNode {
    pub identifier: Identifier,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub program_id: Option<String>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub seeds: Vec<PdaSeedNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl HasIdentifier for PdaNode {
    fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}
