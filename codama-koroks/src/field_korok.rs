use crate::KorokTrait;
use codama_attributes::Attributes;
use codama_errors::CodamaResult;
use codama_nodes::{Node, StructFieldTypeNode, TypeNode};

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

    pub fn set_type_node(&mut self, node: TypeNode) {
        self.node = match &self.ast.ident {
            Some(ident) => Some(StructFieldTypeNode::new(ident.to_string(), node).into()),
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

    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }
}
