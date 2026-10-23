use crate::{CamelCaseString, Docs, HasIdentifier, TypeNode, ValueNode};
use codama_nodes_derive::node;

#[node]
pub struct ConstantNode {
    // Data.
    // TODO(spec v2): keeps the v1 JSON key "name" temporarily so v1 output stays the same. The next v2 regeneration drops it.
    #[serde(rename = "name")]
    pub identifier: CamelCaseString,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,

    // Children.
    pub r#type: Box<TypeNode>,
    pub value: Box<ValueNode>,
}

impl HasIdentifier for ConstantNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
