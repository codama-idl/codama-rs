use crate::{FieldsKorok, KorokTrait};
use codama_attributes::Attributes;
use codama_errors::{combine_errors, CodamaError, CodamaResult};
use codama_nodes::Node;
use codama_syn_helpers::extensions::*;

#[derive(Debug, PartialEq)]
pub struct StructKorok<'a> {
    pub ast: &'a syn::ItemStruct,
    pub attributes: Attributes<'a>,
    pub fields: FieldsKorok<'a>,
    pub node: Option<Node>,
}

impl<'a> StructKorok<'a> {
    pub fn parse(item: &'a syn::Item) -> CodamaResult<Self> {
        let syn::Item::Struct(ast) = item else {
            return Err(item.error("Expected a struct").into());
        };
        let (attributes, fields) = combine_errors!(
            Attributes::parse(&ast.attrs, item.into()).map_err(CodamaError::from),
            FieldsKorok::parse(&ast.fields),
        )?;
        Ok(Self {
            ast,
            attributes,
            fields,
            node: None,
        })
    }
}

impl KorokTrait for StructKorok<'_> {
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
