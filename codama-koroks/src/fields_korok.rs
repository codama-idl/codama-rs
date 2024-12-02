use crate::FieldKorok;
use codama_errors::CodamaResult;
use codama_nodes::Node;

#[derive(Debug, PartialEq)]
pub struct FieldsKorok<'a> {
    pub ast: &'a syn::Fields,
    pub all: Vec<FieldKorok<'a>>,
    pub node: Option<Node>,
}

impl<'a> FieldsKorok<'a> {
    pub fn parse(ast: &'a syn::Fields) -> CodamaResult<Self> {
        Ok(Self {
            ast,
            all: match ast {
                syn::Fields::Named(f) => f.named.iter().map(FieldKorok::parse).collect(),
                syn::Fields::Unnamed(f) => f.unnamed.iter().map(FieldKorok::parse).collect(),
                syn::Fields::Unit => Ok(vec![]),
            }?,
            node: None,
        })
    }
}
