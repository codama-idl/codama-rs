use crate::{attributes::Attribute, TypeKorok};
use codama_errors::CodamaResult;
use codama_nodes::Node;

#[derive(Debug)]
pub struct FieldKorok<'a> {
    pub ast: &'a syn::Field,
    pub attributes: Vec<Attribute<'a>>,
    pub node: Option<Node>,
    pub r#type: TypeKorok<'a>,
}

impl<'a> FieldKorok<'a> {
    pub fn parse(ast: &'a syn::Field) -> CodamaResult<Self> {
        let attributes = Attribute::parse_all(&ast.attrs)?;
        Ok(Self {
            ast,
            attributes,
            node: None,
            r#type: TypeKorok::new(&ast.ty),
        })
    }
}
