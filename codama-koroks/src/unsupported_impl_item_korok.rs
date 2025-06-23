use crate::KorokTrait;
use codama_attributes::Attributes;
use codama_errors::CodamaResult;
use codama_nodes::Node;

#[derive(Debug, PartialEq)]
pub struct UnsupportedImplItemKorok<'a> {
    pub ast: &'a syn::ImplItem,
    pub attributes: Attributes<'a>,
    pub node: Option<Node>,
}

impl<'a> UnsupportedImplItemKorok<'a> {
    pub fn parse(ast: &'a syn::ImplItem) -> CodamaResult<Self> {
        let attributes = match ast {
            syn::ImplItem::Const(item) => Attributes::parse(&item.attrs, ast.into())?,
            syn::ImplItem::Fn(item) => Attributes::parse(&item.attrs, ast.into())?,
            syn::ImplItem::Type(item) => Attributes::parse(&item.attrs, ast.into())?,
            syn::ImplItem::Macro(item) => Attributes::parse(&item.attrs, ast.into())?,
            _ => Attributes(Vec::new()),
        };
        Ok(Self {
            ast,
            attributes,
            node: None,
        })
    }
}

impl KorokTrait for UnsupportedImplItemKorok<'_> {
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
