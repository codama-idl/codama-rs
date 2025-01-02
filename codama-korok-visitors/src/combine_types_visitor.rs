use crate::KorokVisitor;
use codama_errors::CodamaResult;
use codama_nodes::{
    DefinedTypeNode, EnumEmptyVariantTypeNode, EnumStructVariantTypeNode, EnumTupleVariantTypeNode,
    EnumTypeNode, EnumVariantTypeNode, HasKind, Node, RegisteredTypeNode, StructTypeNode,
    TupleTypeNode, TypeNode,
};
use codama_syn_helpers::extensions::*;

#[derive(Default)]
pub struct CombineTypesVisitor {
    pub r#override: bool,
}

impl CombineTypesVisitor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl KorokVisitor for CombineTypesVisitor {
    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        if korok.node.is_some() && !self.r#override {
            return Ok(());
        }

        let name = korok.ast.ident.to_string();
        korok.node = match TypeNode::try_from(korok.fields.node.clone()) {
            Ok(TypeNode::Tuple(tuple_node)) if tuple_node.items.len() == 1 => {
                Some(DefinedTypeNode::new(name, tuple_node.items.first().unwrap().clone()).into())
            }
            Ok(type_node) => Some(DefinedTypeNode::new(name, type_node).into()),
            Err(_) => {
                let message = match &korok.fields.node {
                    Some(node) => format!(
                        "Cannot create a `definedTypeNode` from a node of kind `{}`",
                        node.kind()
                    ),
                    _ => "Cannot create a `definedTypeNode` from `None`".to_string(),
                };
                return Err(korok.ast.error(message).into());
            }
        };
        Ok(())
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        if korok.node.is_some() && !self.r#override {
            return Ok(());
        }

        let enum_name = korok.ast.ident.to_string();
        let variants = korok
            .variants
            .iter()
            .filter_map(|variant| match &variant.node {
                Some(Node::Type(RegisteredTypeNode::EnumEmptyVariant(node))) => {
                    Some(EnumVariantTypeNode::Empty(node.clone()))
                }
                Some(Node::Type(RegisteredTypeNode::EnumTupleVariant(node))) => {
                    Some(EnumVariantTypeNode::Tuple(node.clone()))
                }
                Some(Node::Type(RegisteredTypeNode::EnumStructVariant(node))) => {
                    Some(EnumVariantTypeNode::Struct(node.clone()))
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        korok.node = Some(DefinedTypeNode::new(enum_name, EnumTypeNode::new(variants)).into());
        Ok(())
    }

    fn visit_enum_variant(
        &mut self,
        korok: &mut codama_koroks::EnumVariantKorok,
    ) -> CodamaResult<()> {
        self.visit_children(korok)?;
        if korok.node.is_some() && !self.r#override {
            return Ok(());
        }

        let variant_name = korok.ast.ident.to_string();
        let discriminator = korok
            .ast
            .discriminant
            .as_ref()
            .and_then(|(_, x)| x.as_literal_integer::<usize>().ok());

        korok.node = match (&korok.ast.fields, &korok.fields.node) {
            (syn::Fields::Unit, _) => Some(
                EnumEmptyVariantTypeNode {
                    name: variant_name.into(),
                    discriminator,
                }
                .into(),
            ),
            (syn::Fields::Named(_), Some(Node::Type(RegisteredTypeNode::Struct(node)))) => Some(
                EnumStructVariantTypeNode {
                    name: variant_name.into(),
                    r#struct: node.clone().into(),
                    discriminator,
                }
                .into(),
            ),
            (syn::Fields::Unnamed(_), Some(Node::Type(RegisteredTypeNode::Tuple(node)))) => Some(
                EnumTupleVariantTypeNode {
                    name: variant_name.into(),
                    tuple: node.clone().into(),
                    discriminator,
                }
                .into(),
            ),
            (syn::Fields::Named(_), _) => {
                return Err(korok
                    .ast
                    .error(format!(
                        "Invalid node for enum variant `{}`. Expected a struct node.",
                        korok.ast.ident
                    ))
                    .into())
            }
            (syn::Fields::Unnamed(_), _) => {
                return Err(korok
                    .ast
                    .error(format!(
                        "Invalid node for enum variant `{}`. Expected a tuple node.",
                        korok.ast.ident
                    ))
                    .into())
            }
        };
        Ok(())
    }

    fn visit_fields(&mut self, korok: &mut codama_koroks::FieldsKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        if korok.node.is_some() && !self.r#override {
            return Ok(());
        }

        korok.node = match &korok.ast {
            syn::Fields::Named(_) => {
                let fields = korok
                    .all
                    .iter()
                    .filter_map(|field| match &field.node {
                        Some(Node::Type(RegisteredTypeNode::StructField(field))) => {
                            Some(field.clone())
                        }
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                Some(StructTypeNode::new(fields).into())
            }
            syn::Fields::Unnamed(_) => {
                let items = korok
                    .all
                    .iter()
                    .filter_map(|f| TypeNode::try_from(f.node.clone()).ok())
                    .collect::<Vec<_>>();
                Some(TupleTypeNode::new(items).into())
            }
            syn::Fields::Unit => Some(StructTypeNode::new(vec![]).into()),
        };
        Ok(())
    }
}
