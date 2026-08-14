use crate::KorokVisitor;
use codama_attributes::{
    Attribute, CodamaAttribute, CodamaDirective, DisplayDirective, TryFromFilter,
};
use codama_errors::CodamaResult;
use codama_koroks::{FieldKorok, KorokMut, KorokTrait};
use codama_nodes::{
    HasKind, Node, NumberDisplayNode, RegisteredTypeNode, StructFieldDisplayNode, TypeNode,
};
use codama_syn_helpers::extensions::ToTokensExtension;

#[derive(Default)]
pub struct ApplyDisplayVisitor;

impl ApplyDisplayVisitor {
    pub fn new() -> Self {
        Self
    }
}

impl KorokVisitor for ApplyDisplayVisitor {
    fn visit_field(&mut self, korok: &mut codama_koroks::FieldKorok) -> CodamaResult<()> {
        apply_displays(korok.into())
    }
}

fn apply_displays(mut korok: KorokMut) -> CodamaResult<()> {
    let Some(attributes) = korok.attributes() else {
        return Ok(());
    };
    if attributes.get_first(DisplayDirective::filter).is_none() {
        return Ok(());
    }

    let node = attributes
        .iter()
        .filter_map(|attribute| match attribute {
            Attribute::Codama(attribute) => Some(attribute),
            _ => None,
        })
        .try_fold(korok.node().clone(), |node, attribute| {
            let CodamaDirective::Display(directive) = attribute.directive.as_ref() else {
                return Ok(node);
            };
            apply_display(node, directive, attribute)
        })?;

    korok.set_node(node);
    Ok(())
}

fn apply_display(
    node: Option<Node>,
    directive: &DisplayDirective,
    attribute: &CodamaAttribute,
) -> CodamaResult<Option<Node>> {
    let Some(node) = node else {
        return Err(attribute
            .ast
            .error("Cannot apply attribute `#[codama(display)]` on an empty node")
            .into());
    };

    match node {
        Node::Type(RegisteredTypeNode::StructField(mut field)) => {
            apply_struct_field_display(&mut field.display, directive);
            apply_optional_number_display(&mut field.r#type, directive, attribute)?;
            Ok(Some(field.into()))
        }
        Node::InstructionArgument(mut argument) => {
            apply_struct_field_display(&mut argument.display, directive);
            apply_optional_number_display(&mut argument.r#type, directive, attribute)?;
            Ok(Some(argument.into()))
        }
        node => apply_display_to_type_node(node, directive, attribute).map(Some),
    }
}

fn apply_display_to_type_node(
    node: Node,
    directive: &DisplayDirective,
    attribute: &CodamaAttribute,
) -> CodamaResult<Node> {
    let Some(display) = &directive.number_display else {
        return Ok(node);
    };
    let kind = node.kind();
    let mut type_node = TypeNode::try_from(node).map_err(|_| {
        attribute.ast.error(format!(
            "Cannot apply attribute `#[codama(display)]` as a number display on a node of kind `{kind}`"
        ))
    })?;
    apply_number_display(&mut type_node, display, attribute)?;
    Ok(type_node.into())
}

fn apply_struct_field_display(
    display: &mut Option<StructFieldDisplayNode>,
    directive: &DisplayDirective,
) {
    if directive.label.is_none()
        && directive.skip.is_none()
        && directive.flatten.is_none()
        && directive.flatten_prefix.is_none()
    {
        return;
    }

    let display = display.get_or_insert_default();
    if let Some(label) = &directive.label {
        display.label = Some(label.clone());
    }
    if let Some(skip) = directive.skip {
        display.skip = Some(skip);
    }
    if let Some(flatten) = directive.flatten {
        display.flatten = Some(flatten);
    }
    if let Some(flatten_prefix) = &directive.flatten_prefix {
        display.flatten_prefix = Some(flatten_prefix.clone());
    }
}

pub(crate) fn parse_field_display(field: &FieldKorok) -> Option<StructFieldDisplayNode> {
    // Unnamed fields cannot carry StructFieldDisplayNode metadata during field traversal.
    // Recover it here when a tuple variant turns those fields into named struct fields.
    let mut display = None;
    for directive in field.attributes.get_all(DisplayDirective::filter) {
        apply_struct_field_display(&mut display, directive);
    }
    display
}

fn apply_optional_number_display(
    type_node: &mut TypeNode,
    directive: &DisplayDirective,
    attribute: &CodamaAttribute,
) -> CodamaResult<()> {
    let Some(display) = &directive.number_display else {
        return Ok(());
    };
    apply_number_display(type_node, display, attribute)
}

fn apply_number_display(
    type_node: &mut TypeNode,
    display: &NumberDisplayNode,
    attribute: &CodamaAttribute,
) -> CodamaResult<()> {
    // Follow only value-bearing edges. Nested type modifiers preserve the decoded value,
    // option encodings expose one optional payload, and arrays apply presentation per item.
    // Framing children and semantic number wrappers must keep their own display semantics.
    match type_node {
        TypeNode::Number(number) => {
            *number.display = Some(display.clone());
            Ok(())
        }
        TypeNode::FixedSize(node) => apply_number_display(&mut node.r#type, display, attribute),
        TypeNode::SizePrefix(node) => apply_number_display(&mut node.r#type, display, attribute),
        TypeNode::PreOffset(node) => apply_number_display(&mut node.r#type, display, attribute),
        TypeNode::PostOffset(node) => apply_number_display(&mut node.r#type, display, attribute),
        TypeNode::HiddenPrefix(node) => apply_number_display(&mut node.r#type, display, attribute),
        TypeNode::HiddenSuffix(node) => apply_number_display(&mut node.r#type, display, attribute),
        TypeNode::Sentinel(node) => apply_number_display(&mut node.r#type, display, attribute),
        TypeNode::Option(node) => apply_number_display(&mut node.item, display, attribute),
        TypeNode::RemainderOption(node) => {
            apply_number_display(&mut node.item, display, attribute)
        }
        TypeNode::ZeroableOption(node) => {
            apply_number_display(&mut node.item, display, attribute)
        }
        TypeNode::Array(node) => apply_number_display(&mut node.item, display, attribute),
        node => Err(attribute
            .ast
            .error(format!(
                "Cannot apply attribute `#[codama(display)]` as a number display on a node of kind `{}`",
                node.kind()
            ))
            .into()),
    }
}
