use crate::Node;

#[derive(Debug)]
pub struct PdaNode {
    pub name: String,
}

impl Node for PdaNode {
    const KIND: &'static str = "pdaNode";
}
