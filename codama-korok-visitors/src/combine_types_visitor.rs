use crate::KorokVisitor;
use codama_nodes::{
    DefinedTypeNode, EnumEmptyVariantTypeNode, EnumStructVariantTypeNode, EnumTupleVariantTypeNode,
    EnumTypeNode, EnumVariantTypeNode, Node, RegisteredTypeNode, StructFieldTypeNode,
    StructTypeNode, TupleTypeNode, TypeNode,
};
use codama_syn_helpers::syn_traits::*;

#[derive(Default)]
pub struct CombineTypesVisitor {
    pub r#override: bool,
}

impl CombineTypesVisitor {
    pub fn new() -> Self {
        Self { r#override: false }
    }
}

impl KorokVisitor for CombineTypesVisitor {
    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) {
        self.visit_children(korok);
        if korok.node.is_some() && !self.r#override {
            return ();
        }

        let name = korok.ast.ident.to_string();
        korok.node = match TypeNode::try_from(korok.fields.node.clone()) {
            Ok(TypeNode::Tuple(tuple_node)) if tuple_node.items.len() == 1 => {
                Some(DefinedTypeNode::new(name, tuple_node.items.first().unwrap().clone()).into())
            }
            Ok(type_node) => Some(DefinedTypeNode::new(name, type_node).into()),
            Err(_) => None,
        }
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) {
        self.visit_children(korok);
        if korok.node.is_some() && !self.r#override {
            return ();
        }

        // Ensure all variants have nodes.
        if !korok.variants.iter().all(|field| field.node.is_some()) {
            return ();
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
    }

    fn visit_enum_variant(&mut self, korok: &mut codama_koroks::EnumVariantKorok) {
        self.visit_children(korok);
        if korok.node.is_some() && !self.r#override {
            return ();
        }

        let variant_name = korok.ast.ident.to_string();
        let discriminator = korok
            .ast
            .discriminant
            .as_ref()
            .map_or(None, |(_, x)| x.as_literal_integer::<usize>().ok());

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
            _ => None,
        }
    }

    fn visit_fields(&mut self, korok: &mut codama_koroks::FieldsKorok) {
        self.visit_children(korok);
        if korok.node.is_some() && !self.r#override {
            return ();
        }

        // Ensure all fields have nodes.
        if !korok.all.iter().all(|field| field.node.is_some()) {
            return ();
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
            syn::Fields::Unit => None,
        };
    }

    fn visit_field(&mut self, korok: &mut codama_koroks::FieldKorok) {
        self.visit_children(korok);
        if korok.node.is_some() && !self.r#override {
            return ();
        }

        korok.node = match &korok.r#type.node {
            Some(Node::Type(node)) => match &korok.ast.ident {
                Some(ident) => match TypeNode::try_from(node.clone()) {
                    Ok(node) => Some(StructFieldTypeNode::new(ident.to_string(), node).into()),
                    Err(_) => None,
                },
                None => Some(node.clone().into()),
            },
            _ => None,
        }
    }
}
