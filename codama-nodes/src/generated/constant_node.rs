use crate::{Docs, HasIdentifier, Identifier, PluginNode, TypeNode, ValueNode};
use codama_nodes_derive::node;

#[node]
pub struct ConstantNode {
    pub identifier: Identifier,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,
    pub r#type: Box<TypeNode>,
    pub value: Box<ValueNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl HasIdentifier for ConstantNode {
    fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}
