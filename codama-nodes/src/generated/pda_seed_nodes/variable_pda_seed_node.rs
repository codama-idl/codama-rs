use crate::{Docs, HasIdentifier, Identifier, PluginNode, TypeNode};
use codama_nodes_derive::node;

#[node]
pub struct VariablePdaSeedNode {
    pub identifier: Identifier,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,
    pub r#type: Box<TypeNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<VariablePdaSeedNode> for crate::Node {
    fn from(val: VariablePdaSeedNode) -> Self {
        crate::Node::PdaSeed(val.into())
    }
}

impl HasIdentifier for VariablePdaSeedNode {
    fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}
