use crate::{CamelCaseString, DefinedTypeLinkNode, EnumValuePayload};
use codama_nodes_derive::node;

#[node]
pub struct EnumValueNode {
    // Data.
    pub variant: CamelCaseString,

    // Children.
    pub r#enum: DefinedTypeLinkNode,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub value: Box<Option<EnumValuePayload>>,
}

impl From<EnumValueNode> for crate::Node {
    fn from(val: EnumValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
