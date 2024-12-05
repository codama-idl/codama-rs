use crate::{Korok, TypeKorok};
use codama_attributes::Attributes;
use codama_errors::CodamaResult;
use codama_nodes::Node;

#[derive(Debug, PartialEq)]
pub struct FieldKorok<'a> {
    pub ast: &'a syn::Field,
    pub attributes: Attributes<'a>,
    pub node: Option<Node>,
    pub r#type: TypeKorok<'a>,
}

impl<'a> FieldKorok<'a> {
    pub fn parse(ast: &'a syn::Field) -> CodamaResult<Self> {
        let attributes = Attributes::parse(&ast.attrs)?;
        Ok(Self {
            ast,
            attributes,
            node: None,
            r#type: TypeKorok::new(&ast.ty),
        })
    }
}

impl Korok for FieldKorok<'_> {
    fn node(&self) -> &Option<Node> {
        &self.node
    }

    fn set_node(&mut self, node: Option<Node>) {
        self.node = node;
    }
}
