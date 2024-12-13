use crate::{ApplyToNode, NodeDirective, NumberDirective, StringDirective};
use codama_nodes::Node;
use codama_syn_helpers::{extensions::*, Meta};
use derive_more::derive::From;

#[derive(Debug, PartialEq, From)]
pub enum CodamaDirective {
    Node(NodeDirective),
    Number(NumberDirective),
    String(StringDirective),
}

impl ApplyToNode for CodamaDirective {
    fn apply(&self, node: Option<Node>) -> Option<Node> {
        match &self {
            Self::Node(d) => d.apply(node),
            Self::Number(d) => d.apply(node),
            Self::String(d) => d.apply(node),
        }
    }
}

impl TryFrom<&Meta> for CodamaDirective {
    type Error = syn::Error;

    fn try_from(meta: &Meta) -> syn::Result<Self> {
        let list = meta.as_list()?;
        match list.path.to_string().as_str() {
            "node" => Ok(CodamaDirective::Node(meta.try_into()?)),
            "number" => Ok(CodamaDirective::Number(meta.try_into()?)),
            "string" => Ok(CodamaDirective::String(meta.try_into()?)),
            _ => Err(list.path.error("unrecognized codama directive")),
        }
    }
}
