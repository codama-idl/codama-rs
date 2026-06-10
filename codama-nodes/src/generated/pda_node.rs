use crate::{CamelCaseString, Docs, HasName, PdaSeedNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct PdaNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub program_id: Option<String>,

    // Children.
    pub seeds: Vec<PdaSeedNode>,
}

impl HasName for PdaNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
