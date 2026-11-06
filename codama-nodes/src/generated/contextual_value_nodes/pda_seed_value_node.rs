use crate::{CamelCaseString, HasIdentifier, PdaSeedValueValue, PluginNode};
use codama_nodes_derive::node;

#[node]
pub struct PdaSeedValueNode {
    // Data.
    pub identifier: CamelCaseString,

    // Children.
    pub value: Box<PdaSeedValueValue>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<PdaSeedValueNode> for crate::Node {
    fn from(val: PdaSeedValueNode) -> Self {
        crate::Node::ContextualValue(val.into())
    }
}

impl HasIdentifier for PdaSeedValueNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
