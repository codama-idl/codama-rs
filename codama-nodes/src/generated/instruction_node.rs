use crate::{
    CamelCaseString, DiscriminatorNode, Docs, HasIdentifier, InstructionAccountNode,
    InstructionByteDeltaNode, InstructionDisplayNode, InstructionRemainingAccountsNode,
    InstructionStatusNode, OptionalAccountStrategy, PluginNode, ProvidedNode, TypeNode,
};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct InstructionNode {
    // Data.
    pub identifier: CamelCaseString,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub optional_account_strategy: Option<OptionalAccountStrategy>,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub accounts: Vec<InstructionAccountNode>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub data: Box<Option<TypeNode>>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub return_data: Box<Option<TypeNode>>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub remaining_accounts: Vec<InstructionRemainingAccountsNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub byte_deltas: Vec<InstructionByteDeltaNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub discriminators: Vec<DiscriminatorNode>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub status: Option<InstructionStatusNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub sub_instructions: Vec<InstructionNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub provides: Vec<ProvidedNode>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub display: Option<InstructionDisplayNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl HasIdentifier for InstructionNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
