use crate::{InjectableNumberValueNode, InjectableStringValueNode};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct AmountNumberDisplayNode {
    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub decimals: Box<Option<InjectableNumberValueNode>>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub unit: Box<Option<InjectableStringValueNode>>,
}

impl From<AmountNumberDisplayNode> for crate::Node {
    fn from(val: AmountNumberDisplayNode) -> Self {
        crate::Node::Display(val.into())
    }
}
