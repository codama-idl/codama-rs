use crate::{
    DefaultValueStrategy, Docs, HasIdentifier, Identifier, PluginNode, StructFieldDisplayNode,
    TypeNode, ValueNode,
};
use codama_nodes_derive::node;

#[node]
pub struct StructFieldTypeNode {
    pub identifier: Identifier,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub default_value_strategy: Option<DefaultValueStrategy>,
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

impl HasIdentifier for StructFieldTypeNode {
    fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}
