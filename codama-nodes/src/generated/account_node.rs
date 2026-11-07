use crate::{
    DiscriminatorNode, Docs, HasIdentifier, Identifier, PdaLinkNode, PluginNode, TypeNode,
};
use codama_nodes_derive::node;

#[node]
pub struct AccountNode {
    pub identifier: Identifier,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub size: Option<u64>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,
    pub data: Box<TypeNode>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub pda: Option<PdaLinkNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub discriminators: Vec<DiscriminatorNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl HasIdentifier for AccountNode {
    fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}
