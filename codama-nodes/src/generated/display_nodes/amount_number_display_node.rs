use crate::{InjectableIntegerValueNode, InjectableUnitValueNode, PluginNode};
use codama_nodes_derive::node;

#[node]
pub struct AmountNumberDisplayNode {
    // Children.
    pub decimals: Box<InjectableIntegerValueNode>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub unit: Box<Option<InjectableUnitValueNode>>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<AmountNumberDisplayNode> for crate::Node {
    fn from(val: AmountNumberDisplayNode) -> Self {
        crate::Node::Display(val.into())
    }
}
