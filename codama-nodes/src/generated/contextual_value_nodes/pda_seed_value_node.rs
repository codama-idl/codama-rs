use crate::{CamelCaseString, HasIdentifier, PdaSeedValueValue};
use codama_nodes_derive::node;

#[node]
pub struct PdaSeedValueNode {
    // Data.
    // TODO(spec v2): keeps the v1 JSON key "name" temporarily so v1 output stays the same. The next v2 regeneration drops it.
    #[serde(rename = "name")]
    pub identifier: CamelCaseString,

    // Children.
    pub value: Box<PdaSeedValueValue>,
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
