use crate::{ConditionalValueCondition, InstructionInputValueNode, ValueNode};
use codama_nodes_derive::node;

#[node]
pub struct ConditionalValueNode {
    // Children.
    pub condition: Box<ConditionalValueCondition>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub value: Box<Option<ValueNode>>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub if_true: Box<Option<InstructionInputValueNode>>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub if_false: Box<Option<InstructionInputValueNode>>,
}

impl From<ConditionalValueNode> for crate::Node {
    fn from(val: ConditionalValueNode) -> Self {
        crate::Node::ContextualValue(val.into())
    }
}
