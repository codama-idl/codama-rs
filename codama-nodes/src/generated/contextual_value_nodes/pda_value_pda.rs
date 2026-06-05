use crate::{CamelCaseString, HasName, PdaLinkNode, PdaNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum PdaValuePda {
    Pda(PdaNode),
    PdaLink(PdaLinkNode),
}

impl HasName for PdaValuePda {
    fn name(&self) -> &CamelCaseString {
        match self {
            PdaValuePda::Pda(node) => node.name(),
            PdaValuePda::PdaLink(node) => node.name(),
        }
    }
}
