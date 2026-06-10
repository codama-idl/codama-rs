use crate::{CamelCaseString, Docs, HasName, TypeNode, ValueNode};
use codama_nodes_derive::node;

#[node]
pub struct ConstantNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,

    // Children.
    pub r#type: Box<TypeNode>,
    pub value: Box<ValueNode>,
}

impl HasName for ConstantNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
