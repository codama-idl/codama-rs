use crate::DisplaySkip;
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct StructFieldDisplayNode {
    // Data.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub skip: Option<DisplaySkip>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub flatten: Option<bool>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub flatten_prefix: Option<String>,
}

impl From<StructFieldDisplayNode> for crate::Node {
    fn from(val: StructFieldDisplayNode) -> Self {
        crate::Node::Display(val.into())
    }
}
