use codama_nodes_derive::type_node;

#[type_node]
#[derive(Copy, Default)]
pub struct PublicKeyTypeNode {}

impl From<PublicKeyTypeNode> for crate::Node {
    fn from(val: PublicKeyTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}
