use crate::{
    CamelCaseString, DiscriminatorNode, Docs, HasIdentifier, NestedTypeNode, PdaLinkNode,
    StructTypeNode,
};
use codama_nodes_derive::node;

#[node]
pub struct AccountNode {
    // Data.
    // TODO(spec v2): keeps the v1 JSON key "name" temporarily so v1 output stays the same. The next v2 regeneration drops it.
    #[serde(rename = "name")]
    pub identifier: CamelCaseString,
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

impl HasIdentifier for AccountNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
