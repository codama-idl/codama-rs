use crate::KorokTrait;
use codama_attributes::{Attributes, IdentifierDirective, TryFromFilter};
use codama_errors::{CodamaResult, IteratorCombineErrors};
use codama_nodes::{Identifier, Node, RegisteredTypeNode, StructFieldTypeNode, TypeNode};
use codama_syn_helpers::extensions::*;

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

    pub fn name(&self) -> CodamaResult<Option<Identifier>> {
        if let Some(directive) = self.attributes.get_last(IdentifierDirective::filter) {
            return Ok(Some(directive.identifier.clone()));
        }
        self.ast
            .ident
            .as_ref()
            .map(|ident| {
                Identifier::from_rust_name(&ident.to_string()).map_err(|e| ident.error(e).into())
            })
            .transpose()
    }

    pub fn get_updated_type_node(&self, node: TypeNode) -> CodamaResult<Option<Node>> {
        Ok(match &self.node {
            Some(Node::Type(RegisteredTypeNode::StructField(field))) => Some(
                StructFieldTypeNode {
                    r#type: Box::new(node),
                    ..field.clone()
                }
                .into(),
            ),
            // Only wrap in StructFieldTypeNode for named fields (those with an AST ident).
            // Tuple fields store the raw TypeNode; callers use field.name() to get custom
            // names from #[codama(identifier = "...")] when needed.
            _ => match self.name()? {
                Some(name) if self.ast.ident.is_some() => {
                    Some(StructFieldTypeNode::new(name, node).into())
                }
                _ => Some(node.into()),
            },
        })
    }

    pub fn set_type_node(&mut self, node: TypeNode) -> CodamaResult<()> {
        self.node = self.get_updated_type_node(node)?;
        Ok(())
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
        Identifier,
        IntegerFormat::{U32, U64},
        IntegerTypeNode,
    };

    #[test]
    fn get_updated_type_node_with_named_none() {
        let korok = FieldKorok {
            ast: &syn::parse_quote! { pub my_field: u32 },
            attributes: Attributes(vec![]),
            node: None,
        };
        assert_eq!(
            korok
                .get_updated_type_node(IntegerTypeNode::le(U32).into())
                .unwrap(),
            Some(StructFieldTypeNode::new("my_field", IntegerTypeNode::le(U32)).into())
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
            korok
                .get_updated_type_node(IntegerTypeNode::le(U32).into())
                .unwrap(),
            Some(IntegerTypeNode::le(U32).into())
        );
    }

    #[test]
    fn get_updated_type_node_with_some() {
        let korok = FieldKorok {
            ast: &syn::parse_quote! { pub my_field: u32 },
            attributes: Attributes(vec![]),
            node: Some(StructFieldTypeNode::new("myNodeName", IntegerTypeNode::le(U32)).into()),
        };
        assert_eq!(
            korok
                .get_updated_type_node(IntegerTypeNode::le(U64).into())
                .unwrap(),
            Some(StructFieldTypeNode::new("myNodeName", IntegerTypeNode::le(U64)).into())
        );
    }

    #[test]
    fn name_from_the_rust_ident() {
        let korok = FieldKorok {
            ast: &syn::parse_quote! { pub my_field: u32 },
            attributes: Attributes(vec![]),
            node: None,
        };
        assert_eq!(korok.name().unwrap(), Some(Identifier::from("my_field")));
    }

    #[test]
    fn name_rejects_an_invalid_rust_ident() {
        let korok = FieldKorok {
            ast: &syn::parse_quote! { pub __: u32 },
            attributes: Attributes(vec![]),
            node: None,
        };
        let error = korok.name().unwrap_err();
        assert_eq!(error.to_string(), "Invalid identifier: __");
    }

    #[test]
    fn name_snake_cases_the_rust_ident() {
        let cases: [(syn::Field, &str); 5] = [
            (syn::parse_quote! { pub max_supply: u32 }, "max_supply"),
            (syn::parse_quote! { pub HTTPServer: u32 }, "http_server"),
            (syn::parse_quote! { pub foo1Bar: u32 }, "foo1_bar"),
            (syn::parse_quote! { pub _reserved: u32 }, "reserved"),
            (syn::parse_quote! { pub r#type: u32 }, "type"),
        ];
        for (ast, expected) in cases {
            let korok = FieldKorok {
                ast: &ast,
                attributes: Attributes(vec![]),
                node: None,
            };
            assert_eq!(
                korok.name().unwrap(),
                Some(expected.parse::<Identifier>().unwrap())
            );
        }
    }

    #[test]
    fn name_rejects_a_non_ascii_rust_ident() {
        let korok = FieldKorok {
            ast: &syn::parse_quote! { pub été: u32 },
            attributes: Attributes(vec![]),
            node: None,
        };
        let error = korok.name().unwrap_err();
        assert_eq!(error.to_string(), "Invalid identifier: \u{e9}t\u{e9}");
    }
}
