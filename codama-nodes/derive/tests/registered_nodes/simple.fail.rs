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
    TypeNode::StructField(StructFieldTypeNode {});
}
