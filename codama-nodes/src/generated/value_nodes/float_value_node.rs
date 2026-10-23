use crate::{Decimal, PluginNode};
use codama_nodes_derive::node;

#[node]
pub struct FloatValueNode {
    // Data.
    pub value: Decimal,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<FloatValueNode> for crate::Node {
    fn from(val: FloatValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
