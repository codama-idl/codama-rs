use crate::{CamelCaseString, Docs, HasIdentifier, PluginNode, TypeNode, ValueNode};
use codama_nodes_derive::node;

#[node]
pub struct ConstantNode {
    // Data.
    pub identifier: CamelCaseString,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,
    pub r#type: Box<TypeNode>,
    pub value: Box<ValueNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl HasIdentifier for ConstantNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
