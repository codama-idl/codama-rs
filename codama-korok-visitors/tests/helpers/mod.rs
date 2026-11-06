use codama_nodes::{InstructionNode, Path, StructFieldTypeNode, TypeNode};

pub fn path(value: &str) -> Path {
    value.parse().unwrap()
}

pub fn get_data_fields(instruction: &InstructionNode) -> &[StructFieldTypeNode] {
    match &*instruction.data {
        Some(TypeNode::Struct(data)) => &data.fields,
        _ => &[],
    }
}
