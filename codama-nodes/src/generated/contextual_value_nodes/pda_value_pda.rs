use crate::{CamelCaseString, HasIdentifier, PdaLinkNode, PdaNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum PdaValuePda {
    Pda(PdaNode),
    PdaLink(PdaLinkNode),
}

impl HasIdentifier for PdaValuePda {
    fn identifier(&self) -> &CamelCaseString {
        match self {
            PdaValuePda::Pda(node) => node.identifier(),
            PdaValuePda::PdaLink(node) => node.identifier(),
        }
    }
}
