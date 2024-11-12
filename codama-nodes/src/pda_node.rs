use crate::NodeTrait;

#[derive(Debug)]
pub struct PdaNode {
    // Data.
    pub name: String,
}

impl NodeTrait for PdaNode {
    const KIND: &'static str = "pdaNode";
}
