use crate::KorokTrait;
use codama_attributes::Attributes;
use codama_errors::CodamaResult;
use codama_nodes::Node;
use codama_syn_helpers::extensions::*;

#[derive(Debug, PartialEq)]
pub struct ImplConstKorok<'a> {
    pub ast: &'a syn::ImplItemConst,
    pub attributes: Attributes<'a>,
    pub node: Option<Node>,
}

impl<'a> ImplConstKorok<'a> {
    pub fn parse(item: &'a syn::ImplItem) -> CodamaResult<Self> {
        let syn::ImplItem::Const(ast) = item else {
            return Err(item.error("Expected a const impl item").into());
        };
        let attributes = Attributes::parse(&ast.attrs, item.into())?;
        Ok(Self {
            ast,
            attributes,
            node: None,
        })
    }
}

impl KorokTrait for ImplConstKorok<'_> {
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
