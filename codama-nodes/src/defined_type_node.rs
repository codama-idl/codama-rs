use crate::NodeTrait;

#[derive(Debug)]
pub struct DefinedTypeNode {
    // Data.
    pub name: String,
}

impl NodeTrait for DefinedTypeNode {
    const KIND: &'static str = "definedTypeNode";
}
