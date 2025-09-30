use crate::KorokTrait;
use codama_attributes::{Attributes, NameDirective, TryFromFilter};
use codama_errors::{CodamaResult, IteratorCombineErrors};
use codama_nodes::{CamelCaseString, Node, RegisteredTypeNode, StructFieldTypeNode, TypeNode};

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

    pub fn parse_all(ast: &'a syn::Fields) -> CodamaResult<Vec<Self>> {
        match ast {
            syn::Fields::Named(f) => f.named.iter().map(Self::parse).collect_and_combine_errors(),
            syn::Fields::Unnamed(f) => f
                .unnamed
                .iter()
                .map(Self::parse)
                .collect_and_combine_errors(),
            syn::Fields::Unit => Ok(vec![]),
        }
    }

    pub fn name(&self) -> Option<CamelCaseString> {
        self.attributes
            .get_last(NameDirective::filter)
            .map(|n| n.name.clone())
            .or_else(|| self.ast.ident.as_ref().map(|i| i.to_string().into()))
    }

    pub fn get_updated_type_node(&self, node: TypeNode) -> Option<Node> {
        match &self.node {
            Some(Node::Type(RegisteredTypeNode::StructField(field))) => Some(
                StructFieldTypeNode {
                    r#type: node,
                    ..field.clone()
                }
                .into(),
            ),
            _ => match self.name() {
                Some(name) => Some(StructFieldTypeNode::new(name, node).into()),
                None => Some(node.into()),
            },
        }
    }

    pub fn set_type_node(&mut self, node: TypeNode) {
        self.node = self.get_updated_type_node(node);
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

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{
        NumberFormat::{U32, U64},
        NumberTypeNode,
    };

    #[test]
    fn get_updated_type_node_with_named_none() {
        let korok = FieldKorok {
            ast: &syn::parse_quote! { pub my_field: u32 },
            attributes: Attributes(vec![]),
            node: None,
        };
        assert_eq!(
            korok.get_updated_type_node(NumberTypeNode::le(U32).into()),
            Some(StructFieldTypeNode::new("my_field", NumberTypeNode::le(U32)).into())
        );
    }

    #[test]
    fn get_updated_type_node_with_unnamed_none() {
        let korok = FieldKorok {
            ast: &syn::parse_quote! { u32 },
            attributes: Attributes(vec![]),
            node: None,
        };
        assert_eq!(
            korok.get_updated_type_node(NumberTypeNode::le(U32).into()),
            Some(NumberTypeNode::le(U32).into())
        );
    }

    #[test]
    fn get_updated_type_node_with_some() {
        let korok = FieldKorok {
            ast: &syn::parse_quote! { pub my_field: u32 },
            attributes: Attributes(vec![]),
            node: Some(StructFieldTypeNode::new("my_node_name", NumberTypeNode::le(U32)).into()),
        };
        assert_eq!(
            korok.get_updated_type_node(NumberTypeNode::le(U64).into()),
            Some(StructFieldTypeNode::new("my_node_name", NumberTypeNode::le(U64)).into())
        );
    }
}
