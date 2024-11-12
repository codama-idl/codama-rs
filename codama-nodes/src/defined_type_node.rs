use crate::Node;

#[derive(Debug)]
pub struct DefinedTypeNode {
    // Data.
    pub name: String,
}

impl Node for DefinedTypeNode {
    const KIND: &'static str = "definedTypeNode";
}
