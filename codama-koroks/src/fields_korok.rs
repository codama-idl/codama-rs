use crate::{FieldKorok, Korok};
use codama_errors::{CodamaResult, IteratorCombineErrors};
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
                syn::Fields::Named(f) => f
                    .named
                    .iter()
                    .map(FieldKorok::parse)
                    .collect_and_combine_errors(),
                syn::Fields::Unnamed(f) => f
                    .unnamed
                    .iter()
                    .map(FieldKorok::parse)
                    .collect_and_combine_errors(),
                syn::Fields::Unit => Ok(vec![]),
            }?,
            node: None,
        })
    }
}

impl Korok for FieldsKorok<'_> {
    fn node(&self) -> &Option<Node> {
        &self.node
    }

    fn set_node(&mut self, node: Option<Node>) {
        self.node = node;
    }
}
