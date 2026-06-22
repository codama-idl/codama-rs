use crate::{
    CamelCaseString, DefaultValueStrategy, Docs, HasName, StructFieldDisplayNode, TypeNode,
    ValueNode,
};
use codama_nodes_derive::node;

#[node]
pub struct StructFieldTypeNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub default_value_strategy: Option<DefaultValueStrategy>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,

    // Children.
    pub r#type: Box<TypeNode>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub default_value: Box<Option<ValueNode>>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub display: Option<StructFieldDisplayNode>,
}

impl From<StructFieldTypeNode> for crate::Node {
    fn from(val: StructFieldTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}

impl HasName for StructFieldTypeNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
