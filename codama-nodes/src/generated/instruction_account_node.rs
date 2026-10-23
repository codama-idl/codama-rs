use crate::{
    AccountLinkNode, CamelCaseString, Docs, HasIdentifier, InstructionAccountDisplayNode,
    InstructionInputValueNode, IsSigner,
};
use codama_nodes_derive::node;

#[node]
pub struct InstructionAccountNode {
    // Data.
    // TODO(spec v2): keeps the v1 JSON key "name" temporarily so v1 output stays the same. The next v2 regeneration drops it.
    #[serde(rename = "name")]
    pub identifier: CamelCaseString,
    pub is_writable: bool,
    pub is_signer: IsSigner,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub is_optional: Option<bool>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub default_value: Box<Option<InstructionInputValueNode>>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub account_link: Option<AccountLinkNode>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub display: Option<InstructionAccountDisplayNode>,
}

impl HasIdentifier for InstructionAccountNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
