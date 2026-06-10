use crate::{CamelCaseString, Docs, HasName, InstructionInputValueNode, IsSigner};
use codama_nodes_derive::node;

#[node]
pub struct InstructionAccountNode {
    // Data.
    pub name: CamelCaseString,
    pub is_writable: bool,
    pub is_signer: IsSigner,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub is_optional: Option<bool>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub default_value: Box<Option<InstructionInputValueNode>>,
}

impl HasName for InstructionAccountNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
