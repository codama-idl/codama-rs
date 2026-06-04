use crate::{ConstantPdaSeedValue, TypeNode};
use codama_nodes_derive::node;

#[node]
pub struct ConstantPdaSeedNode {
    // Children.
    pub r#type: Box<TypeNode>,
    pub value: Box<ConstantPdaSeedValue>,
}

impl From<ConstantPdaSeedNode> for crate::Node {
    fn from(val: ConstantPdaSeedNode) -> Self {
        crate::Node::PdaSeed(val.into())
    }
}
