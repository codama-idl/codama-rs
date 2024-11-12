use super::{number_type_node::NumberTypeNode, string_type_node::StringTypeNode};

#[derive(Debug)]
pub enum TypeNode {
    Number(NumberTypeNode),
    String(StringTypeNode),
}
