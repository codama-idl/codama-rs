use crate::{CamelCaseString, HasName, NestedTypeNode, StructTypeNode};
use codama_nodes_derive::node;

#[node]
pub struct EnumStructVariantTypeNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub discriminator: Option<u32>,

    // Children.
    pub r#struct: NestedTypeNode<StructTypeNode>,
}

impl From<EnumStructVariantTypeNode> for crate::Node {
    fn from(val: EnumStructVariantTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}

impl HasName for EnumStructVariantTypeNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
