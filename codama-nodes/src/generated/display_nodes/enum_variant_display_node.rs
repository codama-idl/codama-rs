use crate::{PluginNode, Text};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct EnumVariantDisplayNode {
    // Data.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub skip_inner_data: Option<bool>,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub label: Option<Text>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<EnumVariantDisplayNode> for crate::Node {
    fn from(val: EnumVariantDisplayNode) -> Self {
        crate::Node::Display(val.into())
    }
}
