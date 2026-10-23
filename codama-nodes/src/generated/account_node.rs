use crate::{
    CamelCaseString, DiscriminatorNode, Docs, HasIdentifier, PdaLinkNode, PluginNode, TypeNode,
};
use codama_nodes_derive::node;

#[node]
pub struct AccountNode {
    // Data.
    pub identifier: CamelCaseString,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub size: Option<u64>,

    // Children.
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
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
