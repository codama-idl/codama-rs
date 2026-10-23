use crate::{
    CamelCaseString, DefaultValueStrategy, Docs, HasIdentifier, StructFieldDisplayNode, TypeNode,
    ValueNode,
};
use codama_nodes_derive::node;

#[node]
pub struct StructFieldTypeNode {
    // Data.
    // TODO(spec v2): keeps the v1 JSON key "name" temporarily so v1 output stays the same. The next v2 regeneration drops it.
    #[serde(rename = "name")]
    pub identifier: CamelCaseString,
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

impl HasIdentifier for StructFieldTypeNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
