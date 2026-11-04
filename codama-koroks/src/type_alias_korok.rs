use crate::KorokTrait;
use codama_attributes::{Attributes, IdentifierDirective, TryFromFilter};
use codama_errors::CodamaResult;
use codama_nodes::{Identifier, Node};
use codama_syn_helpers::extensions::*;

#[derive(Debug, PartialEq)]
pub struct TypeAliasKorok<'a> {
    pub ast: &'a syn::ItemType,
    pub attributes: Attributes<'a>,
    pub node: Option<Node>,
}

impl<'a> TypeAliasKorok<'a> {
    pub fn parse(item: &'a syn::Item) -> CodamaResult<Self> {
        let syn::Item::Type(ast) = item else {
            return Err(item.error("Expected a type alias").into());
        };
        let attributes = Attributes::parse(&ast.attrs, item.into())?;
        Ok(Self {
            ast,
            attributes,
            node: None,
        })
    }

    pub fn name(&self) -> CodamaResult<Identifier> {
        match self.attributes.get_last(IdentifierDirective::filter) {
            Some(directive) => Ok(directive.identifier.clone()),
            None => Identifier::from_rust_name(&self.ast.ident.to_string())
                .map_err(|e| self.ast.ident.error(e).into()),
        }
    }
}

impl KorokTrait for TypeAliasKorok<'_> {
    fn node(&self) -> &Option<Node> {
        &self.node
    }

    fn set_node(&mut self, node: Option<Node>) {
        self.node = node;
    }

    fn attributes(&self) -> Option<&Attributes<'_>> {
        Some(&self.attributes)
    }
}
