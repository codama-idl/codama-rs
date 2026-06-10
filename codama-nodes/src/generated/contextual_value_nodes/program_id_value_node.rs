use codama_nodes_derive::node;

#[node]
#[derive(Copy, Default)]
pub struct ProgramIdValueNode {}

impl From<ProgramIdValueNode> for crate::Node {
    fn from(val: ProgramIdValueNode) -> Self {
        crate::Node::ContextualValue(val.into())
    }
}
