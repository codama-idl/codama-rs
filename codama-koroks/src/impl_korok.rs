use crate::{ImplItemKorok, KorokTrait};
use codama_attributes::Attributes;
use codama_errors::{combine_errors, CodamaError, CodamaResult};
use codama_nodes::Node;
use codama_syn_helpers::extensions::*;

#[derive(Debug, PartialEq)]
pub struct ImplKorok<'a> {
    pub ast: &'a syn::ItemImpl,
    pub attributes: Attributes<'a>,
    pub items: Vec<ImplItemKorok<'a>>,
    pub node: Option<Node>,
}

impl<'a> ImplKorok<'a> {
    pub fn parse(item: &'a syn::Item) -> CodamaResult<Self> {
        let syn::Item::Impl(ast) = item else {
            return Err(item.error("Expected an impl block").into());
        };
        let (attributes, items) = combine_errors!(
            Attributes::parse(&ast.attrs, item.into()).map_err(CodamaError::from),
            ImplItemKorok::parse_all(&ast.items),
        )?;
        Ok(Self {
            ast,
            attributes,
            items,
            node: None,
        })
    }
}

impl KorokTrait for ImplKorok<'_> {
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
