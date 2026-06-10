use crate::{CamelCaseString, HasName, NestedTypeNode, TupleTypeNode};
use codama_nodes_derive::node;

#[node]
pub struct EnumTupleVariantTypeNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub discriminator: Option<u32>,

    // Children.
    pub tuple: NestedTypeNode<TupleTypeNode>,
}

impl From<EnumTupleVariantTypeNode> for crate::Node {
    fn from(val: EnumTupleVariantTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}

impl HasName for EnumTupleVariantTypeNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
