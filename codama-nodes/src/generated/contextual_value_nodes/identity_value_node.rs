use codama_nodes_derive::node;

#[node]
#[derive(Copy, Default)]
pub struct IdentityValueNode {}

impl From<IdentityValueNode> for crate::Node {
    fn from(val: IdentityValueNode) -> Self {
        crate::Node::ContextualValue(val.into())
    }
}
