use crate::{
    AccountLinkNode, Docs, HasIdentifier, Identifier, InstructionAccountDisplayNode,
    InstructionInputValueNode, IsSigner, PluginNode,
};
use codama_nodes_derive::node;

#[node]
pub struct InstructionAccountNode {
    pub identifier: Identifier,
    pub is_writable: bool,
    pub is_signer: IsSigner,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub is_optional: Option<bool>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub default_value: Box<Option<InstructionInputValueNode>>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub account_link: Option<AccountLinkNode>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub display: Option<InstructionAccountDisplayNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl HasIdentifier for InstructionAccountNode {
    fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}
