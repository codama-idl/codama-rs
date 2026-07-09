use crate::{Docs, InstructionAccountDisplayNode, InstructionRemainingAccountsValue, IsSigner};
use codama_nodes_derive::node;

#[node]
pub struct InstructionRemainingAccountsNode {
    // Data.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub is_optional: Option<bool>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub is_signer: Option<IsSigner>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub is_writable: Option<bool>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,

    // Children.
    pub value: Box<InstructionRemainingAccountsValue>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub display: Option<InstructionAccountDisplayNode>,
}
