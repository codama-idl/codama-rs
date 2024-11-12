use codama_nodes_derive::Node;

#[Node("numberTypeNode")]
pub struct NumberTypeNode {}

fn main() {
    assert_eq!(NumberTypeNode::KIND, "numberTypeNode");
}
