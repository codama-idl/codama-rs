use crate::{BytesEncoding, StringDisplayNode};
use codama_nodes_derive::type_node;

#[type_node]
#[derive(Copy)]
pub struct StringTypeNode {
    // Data.
    pub encoding: BytesEncoding,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub display: Option<StringDisplayNode>,
}

impl From<StringTypeNode> for crate::Node {
    fn from(val: StringTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
