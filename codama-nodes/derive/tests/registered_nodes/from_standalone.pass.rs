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
    let node: RegisteredTypeNode = TypeNode::Number(NumberTypeNode {}).into();
    assert!(matches!(node, RegisteredTypeNode::Number(_)));

    let node: RegisteredTypeNode = TypeNode::Struct(StructTypeNode {}).into();
    assert!(matches!(node, RegisteredTypeNode::Struct(_)));
}
