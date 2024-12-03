use crate::{attributes::Attribute, EnumVariantKorok, Korok};
use codama_errors::CodamaResult;
use codama_nodes::Node;

#[derive(Debug, PartialEq)]
pub struct EnumKorok<'a> {
    pub ast: &'a syn::ItemEnum,
    pub attributes: Vec<Attribute<'a>>,
    pub node: Option<Node>,
    pub variants: Vec<EnumVariantKorok<'a>>,
}

impl<'a> EnumKorok<'a> {
    pub fn parse(ast: &'a syn::ItemEnum) -> CodamaResult<Self> {
        Ok(Self {
            ast,
            attributes: Attribute::parse_all(&ast.attrs)?,
            node: None,
            variants: EnumVariantKorok::parse_all(&ast.variants)?,
        })
    }
}

impl Korok for EnumKorok<'_> {
    fn node(&self) -> &Option<Node> {
        &self.node
    }

    fn set_node(&mut self, node: Option<Node>) {
        self.node = node;
    }
}
