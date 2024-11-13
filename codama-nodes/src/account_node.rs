use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
pub struct AccountNode {
    // Data.
    pub name: String,
}
