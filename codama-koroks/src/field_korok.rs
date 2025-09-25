use crate::KorokTrait;
use codama_attributes::{Attributes, NameDirective, TryFromFilter};
use codama_errors::CodamaResult;
use codama_nodes::{CamelCaseString, Node, StructFieldTypeNode, TypeNode};

#[derive(Debug, PartialEq)]
pub struct FieldKorok<'a> {
    pub ast: &'a syn::Field,
    pub attributes: Attributes<'a>,
    pub node: Option<Node>,
}

impl<'a> FieldKorok<'a> {
    pub fn parse(ast: &'a syn::Field) -> CodamaResult<Self> {
        let attributes = Attributes::parse(&ast.attrs, ast.into())?;
        Ok(Self {
            ast,
            attributes,
            node: None,
        })
    }

    pub fn name(&self) -> Option<CamelCaseString> {
        self.attributes
            .get_last(NameDirective::filter)
            .map(|n| n.name.clone())
            .or_else(|| self.ast.ident.as_ref().map(|i| i.to_string().into()))
    }

    pub fn set_type_node(&mut self, node: TypeNode) {
        self.node = match self.name() {
            Some(name) => Some(StructFieldTypeNode::new(name, node).into()),
            None => Some(node.into()),
        }
    }
}

impl KorokTrait for FieldKorok<'_> {
    fn node(&self) -> &Option<Node> {
        &self.node
    }

    fn set_node(&mut self, node: Option<Node>) {
        self.node = node;
    }

    fn attributes(&self) -> Option<&Attributes<'_>> {
        Some(&self.attributes)
    }
}
