use crate::Node;

#[derive(Debug)]
pub struct PdaNode {
    // Data.
    pub name: String,
}

impl Node for PdaNode {
    const KIND: &'static str = "pdaNode";
}
