use crate::{EnumVariantTypeNode, NestedTypeNode, NumberTypeNode};
use codama_nodes_derive::type_node;

#[type_node]
pub struct EnumTypeNode {
    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub variants: Vec<EnumVariantTypeNode>,
    pub size: NestedTypeNode<NumberTypeNode>,
}

impl From<EnumTypeNode> for crate::Node {
    fn from(val: EnumTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
