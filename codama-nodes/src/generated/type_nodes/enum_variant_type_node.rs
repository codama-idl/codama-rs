use crate::{Docs, EnumVariantDisplayNode, HasIdentifier, Identifier, PluginNode, TypeNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct EnumVariantTypeNode {
    pub identifier: Identifier,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub discriminator: Option<u32>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub data: Box<Option<TypeNode>>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub display: Option<EnumVariantDisplayNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<EnumVariantTypeNode> for crate::Node {
    fn from(val: EnumVariantTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}

impl HasIdentifier for EnumVariantTypeNode {
    fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}
