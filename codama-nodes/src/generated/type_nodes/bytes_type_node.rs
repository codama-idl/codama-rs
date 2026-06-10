use codama_nodes_derive::type_node;

#[type_node]
#[derive(Copy, Default)]
pub struct BytesTypeNode {}

impl From<BytesTypeNode> for crate::Node {
    fn from(val: BytesTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
