use crate::StructFieldTypeNode;
use codama_nodes_derive::type_node;

#[type_node]
#[derive(Default)]
pub struct StructTypeNode {
    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub fields: Vec<StructFieldTypeNode>,
}

impl From<StructTypeNode> for crate::Node {
    fn from(val: StructTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
