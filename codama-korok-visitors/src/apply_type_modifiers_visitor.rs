use crate::KorokVisitor;
use codama_attributes::{
    Attribute, CodamaAttribute, CodamaDirective, EncodingDirective, FixedSizeDirective,
    SizePrefixDirective,
};
use codama_errors::CodamaResult;
use codama_koroks::{KorokMut, KorokTrait};
use codama_nodes::{
    FixedSizeTransformNode, HasKind, HasTransforms, Node, RegisteredTypeNode,
    SizePrefixTransformNode, TransformNode, TypeNode,
};
use codama_syn_helpers::extensions::ToTokensExtension;

#[derive(Default)]
pub struct ApplyTypeModifiersVisitor;

impl ApplyTypeModifiersVisitor {
    pub fn new() -> Self {
        Self
    }
}

impl KorokVisitor for ApplyTypeModifiersVisitor {
    fn visit_root(&mut self, korok: &mut codama_koroks::RootKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        Ok(())
    }

    fn visit_crate(&mut self, korok: &mut codama_koroks::CrateKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        apply_type_modifiers(korok.into())
    }

    fn visit_file_module(
        &mut self,
        korok: &mut codama_koroks::FileModuleKorok,
    ) -> CodamaResult<()> {
        self.visit_children(korok)?;
        apply_type_modifiers(korok.into())
    }

    fn visit_module(&mut self, korok: &mut codama_koroks::ModuleKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        apply_type_modifiers(korok.into())
    }

    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        apply_type_modifiers(korok.into())
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        apply_type_modifiers(korok.into())
    }

    fn visit_unsupported_item(
        &mut self,
        korok: &mut codama_koroks::UnsupportedItemKorok,
    ) -> CodamaResult<()> {
        self.visit_children(korok)?;
        apply_type_modifiers(korok.into())
    }

    fn visit_enum_variant(
        &mut self,
        korok: &mut codama_koroks::EnumVariantKorok,
    ) -> CodamaResult<()> {
        self.visit_children(korok)?;
        apply_type_modifiers(korok.into())
    }

    fn visit_field(&mut self, korok: &mut codama_koroks::FieldKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        apply_type_modifiers(korok.into())
    }
}

struct ApplyAttributeInput<'a, 'b> {
    node: Option<Node>,
    attribute: &'b CodamaAttribute<'b>,
    _korok: &'b KorokMut<'a, 'b>,
}

/// Apply codama attributes to the node from the bottom up.
fn apply_type_modifiers(mut korok: KorokMut) -> CodamaResult<()> {
    let Some(attributes) = korok.attributes() else {
        return Ok(());
    };

    let node = attributes
        .iter()
        .filter_map(|attribute| match attribute {
            Attribute::Codama(attribute) => Some(attribute),
            _ => None,
        })
        .try_fold(korok.node().clone(), |current_node, attribute| {
            apply_type_modifier(ApplyAttributeInput {
                node: current_node,
                attribute,
                _korok: &korok,
            })
        })?;

    korok.set_node(node);
    Ok(())
}

fn apply_type_modifier(input: ApplyAttributeInput) -> CodamaResult<Option<Node>> {
    match input.attribute.directive.as_ref() {
        CodamaDirective::Encoding(directive) => apply_encoding_directive(directive, input),
        CodamaDirective::FixedSize(directive) => apply_fixed_size_directive(directive, input),
        CodamaDirective::SizePrefix(directive) => apply_size_prefix_directive(directive, input),
        _ => Ok(input.node),
    }
}

fn apply_encoding_directive(
    directive: &EncodingDirective,
    input: ApplyAttributeInput,
) -> CodamaResult<Option<Node>> {
    let ast = input.attribute.ast;
    update_type_node(input, |type_node| match type_node {
        TypeNode::String(mut node) => {
            node.encoding = directive.encoding;
            Ok(node.into())
        }
        node => Err(ast
            .error(format!(
                "Cannot apply attribute `#[codama(encoding)]` on a node of kind `{}`",
                node.kind()
            ))
            .into()),
    })
}

fn apply_fixed_size_directive(
    directive: &FixedSizeDirective,
    input: ApplyAttributeInput,
) -> CodamaResult<Option<Node>> {
    let transform = FixedSizeTransformNode::new(directive.size).into();
    update_type_node(input, |node| Ok(set_size_transform(node, transform)))
}

fn apply_size_prefix_directive(
    directive: &SizePrefixDirective,
    input: ApplyAttributeInput,
) -> CodamaResult<Option<Node>> {
    let transform = SizePrefixTransformNode::new(directive.prefix.clone()).into();
    update_type_node(input, |node| Ok(set_size_transform(node, transform)))
}

/// Replaces the last transform when it sets the size, else appends the new one.
/// So a directive overrides the size that the Rust type implies, such as the `u32` prefix of a `String`.
fn set_size_transform(mut node: TypeNode, new_transform: TransformNode) -> TypeNode {
    let transforms = node.transforms_mut();
    match transforms.last_mut() {
        Some(last @ (TransformNode::FixedSize(_) | TransformNode::SizePrefix(_))) => {
            *last = new_transform
        }
        _ => transforms.push(new_transform),
    }
    node
}

/// Updates the type node within a given `Node` using the provided function.
/// If the `Node` is a `StructFieldTypeNode`, its type node will be updated; otherwise,
/// the function will attempt to update the type node of the `Node` itself.
fn update_type_node(
    input: ApplyAttributeInput,
    update: impl FnOnce(TypeNode) -> CodamaResult<TypeNode>,
) -> CodamaResult<Option<Node>> {
    let Some(node) = input.node else {
        return Err(input
            .attribute
            .ast
            .error(format!(
                "Cannot apply attribute `#[codama({})]` on an empty node",
                input.attribute.directive.name()
            ))
            .into());
    };

    if let Node::Type(RegisteredTypeNode::StructField(mut field)) = node {
        field.r#type = Box::new(update(*field.r#type)?);
        return Ok(Some(field.into()));
    };

    match TypeNode::try_from(node.clone()) {
        Ok(type_node) => Ok(Some(update(type_node)?.into())),
        Err(_) => Err(input
            .attribute
            .ast
            .error(format!(
                "Cannot apply attribute `#[codama({})]` on a node of kind `{}`",
                input.attribute.directive.name(),
                node.kind()
            ))
            .into()),
    }
}
