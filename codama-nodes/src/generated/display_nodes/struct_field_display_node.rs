use crate::{DisplaySkip, PluginNode, Text, ValueLabelDisplayNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct StructFieldDisplayNode {
    // Data.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub skip: Option<DisplaySkip>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub flatten: Option<bool>,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub label: Option<Text>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub flatten_prefix: Option<Text>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub value_labels: Vec<ValueLabelDisplayNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<StructFieldDisplayNode> for crate::Node {
    fn from(val: StructFieldDisplayNode) -> Self {
        crate::Node::Display(val.into())
    }
}
