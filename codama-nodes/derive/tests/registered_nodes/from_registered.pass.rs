use codama_nodes_derive::RegisteredNodes;

pub struct NumberTypeNode {}
pub struct StructTypeNode {}
pub struct StructFieldTypeNode {}

#[derive(RegisteredNodes)]
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
    assert!(matches!(node, Err(())));
}
