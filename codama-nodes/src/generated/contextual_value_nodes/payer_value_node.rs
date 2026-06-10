use codama_nodes_derive::node;

#[node]
#[derive(Copy, Default)]
pub struct PayerValueNode {}

impl From<PayerValueNode> for crate::Node {
    fn from(val: PayerValueNode) -> Self {
        crate::Node::ContextualValue(val.into())
    }
}
