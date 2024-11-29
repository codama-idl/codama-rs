use codama_nodes::{NodeTrait, NodeUnionTrait};
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
    let node = RegisteredTypeNode::Number(NumberTypeNode {});
    assert!(matches!(node, RegisteredTypeNode::Number(_)));

    let node = RegisteredTypeNode::Struct(StructTypeNode {});
    assert!(matches!(node, RegisteredTypeNode::Struct(_)));

    let node = RegisteredTypeNode::StructField(StructFieldTypeNode {});
    assert!(matches!(node, RegisteredTypeNode::StructField(_)));

    let node = TypeNode::Number(NumberTypeNode {});
    assert!(matches!(node, TypeNode::Number(_)));

    let node = TypeNode::Struct(StructTypeNode {});
    assert!(matches!(node, TypeNode::Struct(_)));
}
