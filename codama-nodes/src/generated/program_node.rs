use crate::{
    AccountNode, ConstantNode, DefinedTypeNode, Docs, ErrorNode, EventNode, HasIdentifier,
    Identifier, InstructionNode, PdaNode, PluginNode,
};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct ProgramNode {
    pub identifier: Identifier,
    pub public_key: String,
    pub version: String,
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
    fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}
