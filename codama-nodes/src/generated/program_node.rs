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
    pub accounts: Vec<AccountNode>,
    pub instructions: Vec<InstructionNode>,
    pub defined_types: Vec<DefinedTypeNode>,
    pub pdas: Vec<PdaNode>,
    pub events: Vec<EventNode>,
    pub errors: Vec<ErrorNode>,
    pub constants: Vec<ConstantNode>,
}

impl HasName for ProgramNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
