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
    let node = TypeNode::try_from(RegisteredTypeNode::Number(NumberTypeNode {}));
    assert!(matches!(node, Ok(TypeNode::Number(_))));

    let node = TypeNode::try_from(RegisteredTypeNode::Struct(StructTypeNode {}));
    assert!(matches!(node, Ok(TypeNode::Struct(_))));

    let node = TypeNode::try_from(RegisteredTypeNode::StructField(StructFieldTypeNode {}));
    assert!(matches!(
        node,
        Err(codama_errors::CodamaError::InvalidNodeConversion { .. })
    ));
}
