use codama_nodes::{HasKind, NodeTrait, NodeUnionTrait};
use codama_nodes_derive::{node, node_union, RegisteredNodes};

#[node]
pub struct NumberTypeNode {}

#[node]
pub struct StructTypeNode {}

#[node]
pub struct StructFieldTypeNode {}

#[derive(RegisteredNodes)]
#[node_union]
pub enum RegisteredTypeNode {
    Number(NumberTypeNode),
    Struct(StructTypeNode),
    #[registered]
    StructField(StructFieldTypeNode),
}

fn main() {
    let node: RegisteredTypeNode = TypeNode::Number(NumberTypeNode {}).into();
    assert!(matches!(node, RegisteredTypeNode::Number(_)));

    let node: RegisteredTypeNode = TypeNode::Struct(StructTypeNode {}).into();
    assert!(matches!(node, RegisteredTypeNode::Struct(_)));
}
