use crate::{
    CamelCaseString, Docs, HasIdentifier, InstructionAccountDisplayNode, IsSigner, PluginNode,
};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct InstructionRemainingAccountsNode {
    // Data.
    pub identifier: CamelCaseString,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub is_optional: Option<bool>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub is_signer: Option<IsSigner>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub is_writable: Option<bool>,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub display: Option<InstructionAccountDisplayNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl HasIdentifier for InstructionRemainingAccountsNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
