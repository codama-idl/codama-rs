use crate::PluginNode;
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct StringDisplayNode {
    // Data.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub slice_start: Option<u64>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub slice_end: Option<u64>,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<StringDisplayNode> for crate::Node {
    fn from(val: StringDisplayNode) -> Self {
        crate::Node::Display(val.into())
    }
}
