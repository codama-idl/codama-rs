use crate::{CamelCaseString, HasName};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct EnumEmptyVariantTypeNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub discriminator: Option<u32>,
}

impl From<EnumEmptyVariantTypeNode> for crate::Node {
    fn from(val: EnumEmptyVariantTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}

impl HasName for EnumEmptyVariantTypeNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
