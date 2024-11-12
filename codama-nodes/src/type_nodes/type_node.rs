use super::{NumberTypeNode, StringTypeNode};

#[derive(Debug)]
pub enum TypeNode {
    Number(NumberTypeNode),
    String(StringTypeNode),
}
