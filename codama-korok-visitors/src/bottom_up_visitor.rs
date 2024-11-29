use codama_nodes::{
    DefinedTypeNode, EnumEmptyVariantTypeNode, EnumStructVariantTypeNode, EnumTupleVariantTypeNode,
    Node, RegisteredTypeNode, TypeNode,
};
use codama_syn_helpers::syn_wrap;

use crate::KorokVisitor;

#[derive(Default)]
pub struct BottomUpVisitor {
    pub r#override: bool,
}

impl BottomUpVisitor {
    pub fn new() -> Self {
        Self { r#override: false }
    }
}

impl KorokVisitor for BottomUpVisitor {
    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) {
        self.visit_fields(&mut korok.fields);
        if korok.node.is_some() && !self.r#override {
            return ();
        }

        korok.node = match TypeNode::try_from(korok.fields.node.clone()) {
            Ok(type_node) => {
                let name = korok.ast.ident.to_string();
                Some(DefinedTypeNode::new(name, type_node).into())
            }
            Err(_) => None,
        }
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) {
        for variant_korok in &mut korok.variants {
            self.visit_enum_variant(variant_korok);
        }
        if korok.node.is_some() && !self.r#override {
            return ();
        }

        if !korok.all_variants_have_nodes() {
            return ();
        }
        let enum_name = korok.ast.ident.to_string();
        korok.node = Some(DefinedTypeNode::new(enum_name, korok.create_enum_node()).into());
    }

    fn visit_enum_variant(&mut self, korok: &mut codama_koroks::EnumVariantKorok) {
        self.visit_fields(&mut korok.fields);
        if korok.node.is_some() && !self.r#override {
            return ();
        }

        let variant_name = korok.ast.ident.to_string();
        let discriminator = korok.ast.discriminant.as_ref().map_or(None, |(_, x)| {
            syn_wrap::Expr(&x).as_literal_integer::<usize>().ok()
        });

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
        for field_korok in &mut korok.all {
            self.visit_field(field_korok);
        }
        if korok.node.is_some() && !self.r#override {
            return ();
        }

        if !korok.all_have_nodes() {
            return ();
        }
        korok.node = match &korok.ast {
            syn::Fields::Named(_) => Some(korok.create_struct_node().into()),
            syn::Fields::Unnamed(_) => Some(korok.create_tuple_node().into()),
            syn::Fields::Unit => None,
        };
    }

    fn visit_field(&mut self, korok: &mut codama_koroks::FieldKorok) {
        self.visit_type(&mut korok.r#type);
        if korok.node.is_some() && !self.r#override {
            return ();
        }

        korok.node = korok.create_type_node().map(|node| node.into());
    }
}
