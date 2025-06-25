use crate::KorokTrait;
use codama_attributes::Attributes;
use codama_errors::CodamaResult;
use codama_nodes::Node;
use codama_syn_helpers::extensions::*;

#[derive(Debug, PartialEq)]
pub struct ConstKorok<'a> {
    pub ast: ConstAst<'a>,
    pub attributes: Attributes<'a>,
    pub node: Option<Node>,
}

#[derive(Debug, PartialEq)]
pub enum ConstAst<'a> {
    Item(&'a syn::ItemConst),
    ImplItem(&'a syn::ImplItemConst),
}

impl<'a> ConstKorok<'a> {
    pub fn parse(item: &'a syn::Item) -> CodamaResult<Self> {
        let syn::Item::Const(ast) = item else {
            return Err(item.error("Expected a const item").into());
        };
        let attributes = Attributes::parse(&ast.attrs, item.into())?;
        Ok(Self {
            ast: ConstAst::Item(ast),
            attributes,
            node: None,
        })
    }

    pub fn parse_impl_item(item: &'a syn::ImplItem) -> CodamaResult<Self> {
        let syn::ImplItem::Const(ast) = item else {
            return Err(syn::Error::new_spanned(item, "Expected a const impl item").into());
        };
        let attributes = Attributes::parse(&ast.attrs, item.into())?;
        Ok(Self {
            ast: ConstAst::ImplItem(ast),
            attributes,
            node: None,
        })
    }
}

impl KorokTrait for ConstKorok<'_> {
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
