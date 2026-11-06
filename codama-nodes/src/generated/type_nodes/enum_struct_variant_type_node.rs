use crate::{CamelCaseString, EnumVariantDisplayNode, NestedTypeNode, StructTypeNode};
use codama_nodes_derive::node;

#[node]
pub struct EnumStructVariantTypeNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub discriminator: Option<u32>,

    // Children.
    pub r#struct: NestedTypeNode<StructTypeNode>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub display: Option<EnumVariantDisplayNode>,
}

impl From<EnumStructVariantTypeNode> for crate::Node {
    fn from(val: EnumStructVariantTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
