use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
pub struct DefinedTypeNode {
    // Data.
    pub name: String,
}
