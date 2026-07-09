use crate::{
    CamelCaseString, DefaultValueStrategy, Docs, HasName, InstructionInputValueNode,
    StructFieldDisplayNode, TypeNode,
};
use codama_nodes_derive::node;

#[node]
pub struct InstructionArgumentNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub default_value_strategy: Option<DefaultValueStrategy>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,

    // Children.
    pub r#type: Box<TypeNode>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub default_value: Box<Option<InstructionInputValueNode>>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub display: Option<StructFieldDisplayNode>,
}

impl HasName for InstructionArgumentNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
