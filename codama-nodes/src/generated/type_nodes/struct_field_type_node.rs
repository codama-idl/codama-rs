use crate::{
    CamelCaseString, DefaultValueStrategy, Docs, HasIdentifier, PluginNode, StructFieldDisplayNode,
    TypeNode, ValueNode,
};
use codama_nodes_derive::node;

#[node]
pub struct StructFieldTypeNode {
    // Data.
    pub identifier: CamelCaseString,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub default_value_strategy: Option<DefaultValueStrategy>,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,
    pub r#type: Box<TypeNode>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub default_value: Box<Option<ValueNode>>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub display: Option<StructFieldDisplayNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<StructFieldTypeNode> for crate::Node {
    fn from(val: StructFieldTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}

impl HasIdentifier for StructFieldTypeNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
