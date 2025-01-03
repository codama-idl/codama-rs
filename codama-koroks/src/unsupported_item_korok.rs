use crate::KorokTrait;
use codama_attributes::Attributes;
use codama_errors::CodamaResult;
use codama_nodes::Node;
use codama_syn_helpers::extensions::*;

#[derive(Debug, PartialEq)]
pub struct UnsupportedItemKorok<'a> {
    pub ast: &'a syn::Item,
    pub attributes: Attributes<'a>,
    pub node: Option<Node>,
}

impl<'a> UnsupportedItemKorok<'a> {
    pub fn parse(ast: &'a syn::Item) -> CodamaResult<Self> {
        let attributes = match ast.attributes() {
            Some(attrs) => Attributes::parse(attrs, ast.into())?,
            None => Attributes(Vec::new()),
        };
        Ok(Self {
            ast,
            attributes,
            node: None,
        })
    }
}

impl KorokTrait for UnsupportedItemKorok<'_> {
    fn node(&self) -> &Option<Node> {
        &self.node
    }

    fn set_node(&mut self, node: Option<Node>) {
        self.node = node;
    }

    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }
}
