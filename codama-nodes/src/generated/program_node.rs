use crate::{
    AccountNode, CamelCaseString, ConstantNode, DefinedTypeNode, Docs, ErrorNode, EventNode,
    HasName, InstructionNode, PdaNode, ProgramOrigin,
};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct ProgramNode {
    // Data.
    pub name: CamelCaseString,
    pub public_key: String,
    pub version: String,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub origin: Option<ProgramOrigin>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,

    // Children.
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
}

impl HasName for ProgramNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
