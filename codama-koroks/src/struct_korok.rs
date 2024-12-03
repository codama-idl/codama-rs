use crate::{attributes::Attribute, FieldsKorok, Korok};
use codama_errors::CodamaResult;
use codama_nodes::Node;

#[derive(Debug, PartialEq)]
pub struct StructKorok<'a> {
    pub ast: &'a syn::ItemStruct,
    pub attributes: Vec<Attribute<'a>>,
    pub fields: FieldsKorok<'a>,
    pub node: Option<Node>,
}

impl<'a> StructKorok<'a> {
    pub fn parse(ast: &'a syn::ItemStruct) -> CodamaResult<Self> {
        Ok(Self {
            ast,
            attributes: Attribute::parse_all(&ast.attrs)?,
            fields: FieldsKorok::parse(&ast.fields)?,
            node: None,
        })
    }
}

impl Korok for StructKorok<'_> {
    fn node(&self) -> &Option<Node> {
        &self.node
    }

    fn set_node(&mut self, node: Option<Node>) {
        self.node = node;
    }
}
