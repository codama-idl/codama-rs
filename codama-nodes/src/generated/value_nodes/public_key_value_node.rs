use crate::CamelCaseString;
use codama_nodes_derive::node;

#[node]
pub struct PublicKeyValueNode {
    // Data.
    pub public_key: String,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub identifier: Option<CamelCaseString>,
}

impl From<PublicKeyValueNode> for crate::Node {
    fn from(val: PublicKeyValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
