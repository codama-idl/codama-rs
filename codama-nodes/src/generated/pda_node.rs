use crate::{CamelCaseString, Docs, HasIdentifier, PdaSeedNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct PdaNode {
    // Data.
    // TODO(spec v2): keeps the v1 JSON key "name" temporarily so v1 output stays the same. The next v2 regeneration drops it.
    #[serde(rename = "name")]
    pub identifier: CamelCaseString,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub program_id: Option<String>,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub seeds: Vec<PdaSeedNode>,
}

impl HasIdentifier for PdaNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
