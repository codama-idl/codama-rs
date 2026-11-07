use crate::{HasIdentifier, Identifier, PdaLinkNode, PdaNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum PdaValuePda {
    Pda(PdaNode),
    PdaLink(PdaLinkNode),
}

impl HasIdentifier for PdaValuePda {
    fn identifier(&self) -> &Identifier {
        match self {
            PdaValuePda::Pda(node) => node.identifier(),
            PdaValuePda::PdaLink(node) => node.identifier(),
        }
    }
}
