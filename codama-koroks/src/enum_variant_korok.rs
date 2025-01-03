use crate::{FieldsKorok, KorokTrait};
use codama_attributes::Attributes;
use codama_errors::{combine_errors, CodamaError, CodamaResult, IteratorCombineErrors};
use codama_nodes::Node;

#[derive(Debug, PartialEq)]
pub struct EnumVariantKorok<'a> {
    pub ast: &'a syn::Variant,
    pub attributes: Attributes<'a>,
    pub fields: FieldsKorok<'a>,
    pub node: Option<Node>,
}

impl<'a> EnumVariantKorok<'a> {
    pub fn parse(ast: &'a syn::Variant) -> CodamaResult<Self> {
        let (attributes, fields) = combine_errors!(
            Attributes::parse(&ast.attrs, ast.into()).map_err(CodamaError::from),
            FieldsKorok::parse(&ast.fields),
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
}

impl KorokTrait for EnumVariantKorok<'_> {
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
