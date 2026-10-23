use crate::{CamelCaseString, Docs, HasIdentifier, TypeNode};
use codama_nodes_derive::node;

#[node]
pub struct VariablePdaSeedNode {
    // Data.
    // TODO(spec v2): keeps the v1 JSON key "name" temporarily so v1 output stays the same. The next v2 regeneration drops it.
    #[serde(rename = "name")]
    pub identifier: CamelCaseString,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,

    // Children.
    pub r#type: Box<TypeNode>,
}

impl From<VariablePdaSeedNode> for crate::Node {
    fn from(val: VariablePdaSeedNode) -> Self {
        crate::Node::PdaSeed(val.into())
    }
}

impl HasIdentifier for VariablePdaSeedNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
