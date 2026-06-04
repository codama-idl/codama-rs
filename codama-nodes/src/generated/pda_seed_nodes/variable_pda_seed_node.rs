use crate::{CamelCaseString, Docs, HasName, TypeNode};
use codama_nodes_derive::node;

#[node]
pub struct VariablePdaSeedNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,

    // Children.
    pub r#type: Box<TypeNode>,
}

impl From<VariablePdaSeedNode> for crate::Node {
    fn from(val: VariablePdaSeedNode) -> Self {
        crate::Node::PdaSeed(val.into())
    }
}

impl HasName for VariablePdaSeedNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
