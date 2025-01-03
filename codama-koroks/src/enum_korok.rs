use crate::{EnumVariantKorok, KorokTrait};
use codama_attributes::Attributes;
use codama_errors::{combine_errors, CodamaError, CodamaResult};
use codama_nodes::Node;
use codama_syn_helpers::extensions::*;

#[derive(Debug, PartialEq)]
pub struct EnumKorok<'a> {
    pub ast: &'a syn::ItemEnum,
    pub attributes: Attributes<'a>,
    pub node: Option<Node>,
    pub variants: Vec<EnumVariantKorok<'a>>,
}

impl<'a> EnumKorok<'a> {
    pub fn parse(item: &'a syn::Item) -> CodamaResult<Self> {
        let syn::Item::Enum(ast) = item else {
            return Err(item.error("Expected an enum").into());
        };
        let (attributes, variants) = combine_errors!(
            Attributes::parse(&ast.attrs, item.into()).map_err(CodamaError::from),
            EnumVariantKorok::parse_all(&ast.variants),
        )?;
        Ok(Self {
            ast,
            attributes,
            node: None,
            variants,
        })
    }
}

impl KorokTrait for EnumKorok<'_> {
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
