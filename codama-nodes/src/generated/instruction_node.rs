use crate::{
    CamelCaseString, DiscriminatorNode, Docs, HasIdentifier, InstructionAccountNode,
    InstructionArgumentNode, InstructionByteDeltaNode, InstructionDisplayNode,
    InstructionRemainingAccountsNode, InstructionStatusNode, OptionalAccountStrategy, PluginNode,
    ProvidedNode,
};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct InstructionNode {
    // Data.
    // TODO(spec v2): keeps the v1 JSON key "name" temporarily so v1 output stays the same. The next v2 regeneration drops it.
    #[serde(rename = "name")]
    pub identifier: CamelCaseString,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub optional_account_strategy: Option<OptionalAccountStrategy>,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub accounts: Vec<InstructionAccountNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
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

impl HasIdentifier for InstructionNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
