use crate::Korok;
use codama_attributes::Attributes;
use codama_errors::CodamaResult;
use codama_nodes::Node;
use codama_syn_helpers::syn_traits::*;

#[derive(Debug, PartialEq)]
pub struct UnsupportedItemKorok<'a> {
    pub ast: &'a syn::Item,
    pub attributes: Attributes<'a>,
    pub node: Option<Node>,
}

impl<'a> UnsupportedItemKorok<'a> {
    pub fn parse(ast: &'a syn::Item) -> CodamaResult<Self> {
        Ok(Self {
            ast,
            attributes: Attributes::parse(ast.attributes())?,
            node: None,
        })
    }
}

impl Korok for UnsupportedItemKorok<'_> {
    fn node(&self) -> &Option<Node> {
        &self.node
    }

    fn set_node(&mut self, node: Option<Node>) {
        self.node = node;
    }
}
