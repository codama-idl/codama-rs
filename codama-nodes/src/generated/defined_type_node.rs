use crate::{CamelCaseString, Docs, HasName, TypeNode};
use codama_nodes_derive::node;

#[node]
pub struct DefinedTypeNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,

    // Children.
    pub r#type: Box<TypeNode>,
}

impl HasName for DefinedTypeNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
