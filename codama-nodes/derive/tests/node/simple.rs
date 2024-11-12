use codama_nodes_derive::node;

pub trait NodeTrait {
    const KIND: &'static str;
}

#[node]
pub struct NumberTypeNode {}

fn main() {
    assert_eq!(NumberTypeNode::KIND, "numberTypeNode");
}
