use crate::{FieldKorok, KorokTrait};
use codama_attributes::{Attributes, IdentifierDirective, TryFromFilter};
use codama_errors::{combine_errors, CodamaError, CodamaResult, IteratorCombineErrors};
use codama_nodes::{Identifier, Node};
use codama_syn_helpers::extensions::*;

#[derive(Debug, PartialEq)]
pub struct EnumVariantKorok<'a> {
    pub ast: &'a syn::Variant,
    pub attributes: Attributes<'a>,
    pub fields: Vec<FieldKorok<'a>>,
    pub node: Option<Node>,
}

impl<'a> EnumVariantKorok<'a> {
    pub fn parse(ast: &'a syn::Variant) -> CodamaResult<Self> {
        let (attributes, fields) = combine_errors!(
            Attributes::parse(&ast.attrs, ast.into()).map_err(CodamaError::from),
            FieldKorok::parse_all(&ast.fields),
        )?;
        Ok(Self {
            ast,
            attributes,
            fields,
            node: None,
        })
    }

    pub fn parse_all(
        variants: &'a syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
    ) -> CodamaResult<Vec<Self>> {
        variants
            .iter()
            .map(Self::parse)
            .collect_and_combine_errors()
    }

    pub fn name(&self) -> CodamaResult<Identifier> {
        match self.attributes.get_last(IdentifierDirective::filter) {
            Some(directive) => Ok(directive.identifier.clone()),
            None => Identifier::from_rust_name(&self.ast.ident.to_string())
                .map_err(|e| self.ast.ident.error(e).into()),
        }
    }
}

impl KorokTrait for EnumVariantKorok<'_> {
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
