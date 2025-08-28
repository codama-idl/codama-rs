use crate::{ConstKorok, KorokTrait, UnsupportedImplItemKorok};
use codama_attributes::Attributes;
use codama_errors::{CodamaResult, IteratorCombineErrors};
use codama_nodes::Node;

#[derive(Debug, PartialEq)]
pub enum ImplItemKorok<'a> {
    Const(ConstKorok<'a>),
    Unsupported(UnsupportedImplItemKorok<'a>),
}

impl<'a> ImplItemKorok<'a> {
    pub fn parse(item: &'a syn::ImplItem) -> CodamaResult<Self> {
        match item {
            syn::ImplItem::Const(_) => Ok(ImplItemKorok::Const(ConstKorok::parse_impl_item(item)?)),
            _ => Ok(ImplItemKorok::Unsupported(UnsupportedImplItemKorok::parse(
                item,
            )?)),
        }
    }

    pub fn parse_all(items: &'a [syn::ImplItem]) -> CodamaResult<Vec<Self>> {
        items.iter().map(Self::parse).collect_and_combine_errors()
    }
}

impl KorokTrait for ImplItemKorok<'_> {
    fn node(&self) -> &Option<Node> {
        match self {
            ImplItemKorok::Const(k) => k.node(),
            ImplItemKorok::Unsupported(k) => k.node(),
        }
    }

    fn set_node(&mut self, node: Option<Node>) {
        match self {
            ImplItemKorok::Const(k) => k.set_node(node),
            ImplItemKorok::Unsupported(k) => k.set_node(node),
        }
    }

    fn attributes(&self) -> Option<&Attributes<'_>> {
        match self {
            ImplItemKorok::Const(k) => k.attributes(),
            ImplItemKorok::Unsupported(k) => k.attributes(),
        }
    }
}
