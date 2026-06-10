use crate::{
    CamelCaseString, DiscriminatorNode, Docs, HasName, NestedTypeNode, PdaLinkNode, StructTypeNode,
};
use codama_nodes_derive::node;

#[node]
pub struct AccountNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub size: Option<u64>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,

    // Children.
    pub data: NestedTypeNode<StructTypeNode>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub pda: Option<PdaLinkNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub discriminators: Vec<DiscriminatorNode>,
}

impl HasName for AccountNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
