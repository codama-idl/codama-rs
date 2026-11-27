//! What the visitors turning the variants of an enum into program items share: the numbering of the
//! variants, and the struct data of a struct or a variant.

use crate::parse_field_display;
use codama_attributes::{DefaultValueDirective, EnumDiscriminatorDirective, TryFromFilter};
use codama_errors::CodamaResult;
use codama_koroks::{EnumVariantKorok, FieldKorok, StructKorok};
use codama_nodes::{
    DefaultValueStrategy, EnumVariantTypeNode, FieldDiscriminatorNode, Identifier,
    IntegerValueNode, Node, StructFieldTypeNode, StructTypeNode, TypeNode, ValueNode,
};
use codama_syn_helpers::extensions::*;
use std::fmt;

/// The kind of program item an enum or a struct is turned into, for error messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ItemKind {
    Account,
    Event,
    Instruction,
}

impl fmt::Display for ItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Account => "an account",
            Self::Event => "an event",
            Self::Instruction => "an instruction",
        })
    }
}

/// The enum being visited by a visitor turning its variants into program items: its identifier, its
/// discriminator directive and the discriminator of the next variant, which Rust numbers from the
/// previous one.
#[derive(Debug, Default)]
pub(crate) struct EnumItemContext {
    pub identifier: String,
    pub discriminator: EnumDiscriminatorDirective,
    /// The discriminator of the previous variant, if any.
    previous_discriminator: Option<u32>,
}

impl EnumItemContext {
    pub(crate) fn new(identifier: String, discriminator: EnumDiscriminatorDirective) -> Self {
        Self {
            identifier,
            discriminator,
            previous_discriminator: None,
        }
    }

    /// The discriminator of `variant`, which Rust gives it: the one written, or else one more than
    /// the previous variant.
    /// The variant following `u32::MAX` has no discriminator to take, which is an error, but a last
    /// variant at `u32::MAX` is fine.
    pub(crate) fn next_discriminator(&mut self, variant: &EnumVariantKorok) -> CodamaResult<u32> {
        let discriminator = match (&variant.ast.discriminant, self.previous_discriminator) {
            (Some((_, expr)), _) => expr.as_unsigned_integer()?,
            (None, None) => 0,
            (None, Some(previous)) => previous.checked_add(1).ok_or_else(|| {
                variant.ast.error(format!(
                    "the discriminator of `{}` overflows a u32",
                    variant.ast.ident
                ))
            })?,
        };
        self.previous_discriminator = Some(discriminator);
        Ok(discriminator)
    }

    /// The field holding the discriminator of a variant, with the given value, to prepend to its data.
    pub(crate) fn discriminator_field(&self, discriminator: u32) -> StructFieldTypeNode {
        StructFieldTypeNode {
            default_value_strategy: Some(DefaultValueStrategy::Omitted),
            default_value: Box::new(Some(IntegerValueNode::new(discriminator).into())),
            ..StructFieldTypeNode::from(&self.discriminator)
        }
    }

    /// The discriminator node of the items, which reads the discriminator field.
    pub(crate) fn discriminator_node(&self) -> FieldDiscriminatorNode {
        FieldDiscriminatorNode::from(&self.discriminator)
    }
}

/// The identifier and struct data of a struct turned into a program item, from its defined type node.
pub(crate) fn parse_struct_data(
    korok: &StructKorok,
    kind: ItemKind,
) -> CodamaResult<(Identifier, StructTypeNode)> {
    if let Some(Node::DefinedType(node)) = &korok.node {
        if let TypeNode::Struct(data) = (*node.r#type).clone() {
            return Ok((node.identifier.clone(), data));
        }
    }
    Err(korok
        .ast
        .error(format!(
            "The `{}` struct could not be used as {kind} because its type is not a `structTypeNode`.",
            korok.ast.ident
        ))
        .into())
}

/// The identifier and struct data of a variant turned into a program item. A unit variant has no
/// fields, and the items of a tuple variant become fields named after `identifier = "..."` or else
/// `arg0`, `arg1` and so on.
pub(crate) fn parse_variant_data(
    korok: &EnumVariantKorok,
    context: &EnumItemContext,
    kind: ItemKind,
) -> CodamaResult<(Identifier, StructTypeNode)> {
    let variant = korok
        .node
        .clone()
        .and_then(|node| EnumVariantTypeNode::try_from(node).ok());
    let data = match variant.as_ref().map(|variant| variant.data.as_ref()) {
        Some(None) => Some(StructTypeNode::new(vec![])),
        Some(Some(TypeNode::Struct(data))) => Some(data.clone()),
        // The items of a tuple variant become struct fields, which take the display and the value
        // default of their Rust field like the named fields of a struct variant do.
        Some(Some(TypeNode::Tuple(tuple))) => Some(StructTypeNode::new(
            tuple
                .items
                .iter()
                .enumerate()
                .map(|(index, item)| {
                    let field = korok.fields.get(index);
                    let (default_value, default_value_strategy) = field
                        .map(value_default)
                        .transpose()?
                        .flatten()
                        .map(|(value, strategy)| (Some(value), strategy))
                        .unwrap_or_default();
                    Ok(StructFieldTypeNode {
                        display: field.and_then(parse_field_display),
                        default_value: Box::new(default_value),
                        default_value_strategy,
                        ..StructFieldTypeNode::new(
                            data_field_identifier(&korok.fields, index)?,
                            item.clone(),
                        )
                    })
                })
                .collect::<CodamaResult<Vec<_>>>()?,
        )),
        _ => None,
    };
    match (variant, data) {
        (Some(variant), Some(data)) => Ok((variant.identifier, data)),
        _ => Err(korok
            .ast
            .error(format!(
                "The `{}` variant of the `{}` enum could not be used as {kind} because its fields do not resolve to a struct or a tuple.",
                korok.ast.ident, context.identifier
            ))
            .into()),
    }
}

/// The identifier of the data field that the Rust field at `index` gives.
/// A named field takes the identifier of its node, which `type = field("x", ...)` can rename.
/// An unnamed tuple item without `identifier = "..."` gets a synthetic identifier such as `arg0`.
pub(crate) fn data_field_identifier(
    fields: &[FieldKorok],
    index: usize,
) -> CodamaResult<Identifier> {
    let field = fields.get(index);
    match field.and_then(|field| field.node.as_ref()) {
        Some(Node::StructFieldType(node)) => Ok(node.identifier.clone()),
        _ => match field.map(|field| field.name()).transpose()? {
            Some(Some(name)) => Ok(name),
            _ => Identifier::from_rust_name(&format!("arg{index}")),
        },
    }
}

/// The value default written on a Rust field, if any. A contextual default, such as `payer`, is not a
/// value, so it is left to the visitor that knows how to provide it.
fn value_default(
    field: &FieldKorok,
) -> CodamaResult<Option<(ValueNode, Option<DefaultValueStrategy>)>> {
    let Some(directive) = field.attributes.get_last(DefaultValueDirective::filter) else {
        return Ok(None);
    };
    let node = directive.node.try_resolved()?;
    Ok(ValueNode::try_from(node.clone())
        .ok()
        .map(|value| (value, directive.default_value_strategy)))
}
