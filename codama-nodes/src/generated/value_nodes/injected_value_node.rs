use crate::{CamelCaseString, ValueNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct InjectedValueNode {
    // Data.
    pub key: CamelCaseString,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub fallback: Box<Option<ValueNode>>,
}

impl From<InjectedValueNode> for crate::Node {
    fn from(val: InjectedValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
