use crate::{attributes::Attribute, FieldsKorok, Korok};
use codama_errors::CodamaResult;
use codama_nodes::Node;

#[derive(Debug, PartialEq)]
pub struct EnumVariantKorok<'a> {
    pub ast: &'a syn::Variant,
    pub attributes: Vec<Attribute<'a>>,
    pub fields: FieldsKorok<'a>,
    pub node: Option<Node>,
}

impl<'a> EnumVariantKorok<'a> {
    pub fn parse(ast: &'a syn::Variant) -> CodamaResult<Self> {
        Ok(Self {
            ast,
            attributes: Attribute::parse_all(&ast.attrs)?,
            fields: FieldsKorok::parse(&ast.fields)?,
            node: None,
        })
    }

    pub fn parse_all(
        variants: &'a syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
    ) -> CodamaResult<Vec<Self>> {
        variants.iter().map(Self::parse).collect()
    }
}

impl Korok for EnumVariantKorok<'_> {
    fn node(&self) -> &Option<Node> {
        &self.node
    }

    fn set_node(&mut self, node: Option<Node>) {
        self.node = node;
    }
}
