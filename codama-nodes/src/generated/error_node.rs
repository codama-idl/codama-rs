use crate::{CamelCaseString, Docs, HasName};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct ErrorNode {
    // Data.
    pub name: CamelCaseString,
    pub code: u32,
    pub message: String,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,
}

impl HasName for ErrorNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
