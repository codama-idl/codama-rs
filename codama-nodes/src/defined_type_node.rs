use crate::Node;

#[derive(Debug)]
pub struct DefinedTypeNode {
    pub name: String,
}

impl Node for DefinedTypeNode {
    const KIND: &'static str = "definedTypeNode";
}
