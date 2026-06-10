use crate::{ConstantValueNode, TypeNode};
use codama_nodes_derive::type_node;

#[type_node]
pub struct ZeroableOptionTypeNode {
    // Children.
    pub item: Box<TypeNode>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub zero_value: Option<ConstantValueNode>,
}

impl From<ZeroableOptionTypeNode> for crate::Node {
    fn from(val: ZeroableOptionTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
