use crate::{
    AccountNode, CamelCaseString, ConstantNode, DefinedTypeNode, Docs, ErrorNode, EventNode,
    HasIdentifier, InstructionNode, PdaNode, PluginNode,
};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct ProgramNode {
    // Data.
    pub identifier: CamelCaseString,
    pub public_key: String,
    pub version: String,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub accounts: Vec<AccountNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub instructions: Vec<InstructionNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub defined_types: Vec<DefinedTypeNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub pdas: Vec<PdaNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub events: Vec<EventNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub errors: Vec<ErrorNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub constants: Vec<ConstantNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl HasIdentifier for ProgramNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
