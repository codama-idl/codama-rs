use crate::{
    CamelCaseString, DiscriminatorNode, Docs, HasName, InstructionAccountNode,
    InstructionArgumentNode, InstructionByteDeltaNode, InstructionDisplayNode,
    InstructionRemainingAccountsNode, InstructionStatusNode, OptionalAccountStrategy, PluginNode,
    ProvidedNode,
};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct InstructionNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub optional_account_strategy: Option<OptionalAccountStrategy>,

    // Children.
    pub accounts: Vec<InstructionAccountNode>,
    pub arguments: Vec<InstructionArgumentNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub extra_arguments: Vec<InstructionArgumentNode>,
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

impl HasName for InstructionNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
