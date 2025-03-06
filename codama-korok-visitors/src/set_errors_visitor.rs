use crate::{CombineTypesVisitor, KorokVisitor};
use codama_attributes::{Attribute, Attributes, UnsupportedAttribute};
use codama_errors::CodamaResult;
use codama_nodes::{DefinedTypeNode, Docs, ErrorNode, Node, ProgramNode};
use codama_syn_helpers::extensions::*;

pub struct SetErrorsVisitor {
    combine_types: CombineTypesVisitor,
    enum_name: Option<String>,
    enum_current_discriminator: usize,
}

impl Default for SetErrorsVisitor {
    fn default() -> Self {
        Self {
            combine_types: CombineTypesVisitor::strict(),
            enum_name: None,
            enum_current_discriminator: 0,
        }
    }
}

impl SetErrorsVisitor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl KorokVisitor for SetErrorsVisitor {
    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
        // No overrides.
        if korok.node.is_some() {
            return Ok(());
        };

        // Ensure the struct has the `CodamaErrors` attribute.
        if !korok.attributes.has_codama_derive("CodamaErrors") {
            return Ok(());
        };

        // Create a `DefinedTypeNode` from the enum.
        self.combine_types.visit_enum(korok)?;

        // Get details from the defined type enum.
        let enum_name = match &korok.node {
            Some(Node::DefinedType(DefinedTypeNode { name, .. })) => Some(name.to_string()),
            _ => None,
        };

        // Transform each variant into an `ErrorNode`.
        self.enum_name = Some(enum_name.unwrap_or(korok.ast.ident.to_string()));
        self.enum_current_discriminator = 0;
        self.visit_children(korok)?;
        self.enum_name = None;
        self.enum_current_discriminator = 0;

        // Gather all errors in a `ProgramNode`.
        let errors = korok
            .variants
            .iter()
            .filter_map(|variant| match &variant.node {
                Some(Node::Error(error)) => Some(error.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();

        korok.node = Some(
            ProgramNode {
                errors,
                ..ProgramNode::default()
            }
            .into(),
        );

        Ok(())
    }

    fn visit_enum_variant(
        &mut self,
        korok: &mut codama_koroks::EnumVariantKorok,
    ) -> CodamaResult<()> {
        // Update current discriminator.
        let current_discriminator = self.enum_current_discriminator;
        self.enum_current_discriminator = match &korok.ast.discriminant {
            Some((_, expr)) => expr.as_literal_integer()?,
            _ => current_discriminator + 1,
        };

        // Get #[error] attribute message.
        let message = get_message_from_thiserror(&korok.attributes);

        korok.node = Some(
            ErrorNode {
                name: korok.ast.ident.to_string().into(),
                code: current_discriminator,
                message: message.unwrap_or("".to_string()),
                docs: Docs::default(),
            }
            .into(),
        );

        Ok(())
    }
}

pub fn get_message_from_thiserror(attributes: &Attributes) -> Option<String> {
    attributes.iter().find_map(|attr| {
        // Ensure the attribute is a meta list.
        let Attribute::Unsupported(UnsupportedAttribute {
            ast:
                syn::Attribute {
                    meta: syn::Meta::List(list),
                    ..
                },
        }) = attr
        else {
            return None;
        };

        // Ensure the path is `#[error("...")]`.
        if !list.path.is("thiserror::error") {
            return None;
        };

        // Get the first meta as a string, if possible.
        let metas = list.parse_metas().ok()?;
        metas.first()?.as_expr().ok()?.as_literal_string().ok()
    })
}
