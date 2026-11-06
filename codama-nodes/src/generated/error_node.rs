use crate::{CamelCaseString, Docs, HasIdentifier};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct ErrorNode {
    // Data.
    // TODO(spec v2): keeps the v1 JSON key "name" temporarily so v1 output stays the same. The next v2 regeneration drops it.
    #[serde(rename = "name")]
    pub identifier: CamelCaseString,
    pub code: u32,
    pub message: String,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,
}

impl HasIdentifier for ErrorNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
