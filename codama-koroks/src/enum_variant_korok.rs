use crate::{FieldKorok, KorokTrait};
use codama_attributes::{Attributes, NameDirective, TryFromFilter};
use codama_errors::{combine_errors, CodamaError, CodamaResult, IteratorCombineErrors};
use codama_nodes::{CamelCaseString, Node};

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

    pub fn name(&self) -> CamelCaseString {
        self.attributes
            .get_last(NameDirective::filter)
            .map(|n| n.name.clone())
            .unwrap_or(self.ast.ident.to_string().into())
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
