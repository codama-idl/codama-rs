use crate::{PdaSeedValueNode, PdaValuePda, PdaValueProgramId};
use codama_nodes_derive::node;

#[node]
pub struct PdaValueNode {
    // Children.
    pub pda: Box<PdaValuePda>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub seeds: Vec<PdaSeedValueNode>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub program_id: Box<Option<PdaValueProgramId>>,
}

impl From<PdaValueNode> for crate::Node {
    fn from(val: PdaValueNode) -> Self {
        crate::Node::ContextualValue(val.into())
    }
}
