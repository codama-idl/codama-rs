use crate::{InjectableUnitValueNode, PluginNode};
use codama_nodes_derive::node;

#[node]
pub struct UnitNumberDisplayNode {
    // Children.
    pub unit: Box<InjectableUnitValueNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<UnitNumberDisplayNode> for crate::Node {
    fn from(val: UnitNumberDisplayNode) -> Self {
        crate::Node::Display(val.into())
    }
}
