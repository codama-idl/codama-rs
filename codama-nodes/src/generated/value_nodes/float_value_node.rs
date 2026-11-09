use crate::{Decimal, PluginNode};
use codama_nodes_derive::node;

#[node]
pub struct FloatValueNode {
    pub value: Decimal,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}
