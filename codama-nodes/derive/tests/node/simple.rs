use codama_nodes_derive::Node;

pub trait NodeTrait {
    const KIND: &'static str;
}

#[derive(Node)]
pub struct NumberTypeNode {}

fn main() {
    assert_eq!(NumberTypeNode::KIND, "numberTypeNode");
}
