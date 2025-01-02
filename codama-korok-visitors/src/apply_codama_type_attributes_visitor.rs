use crate::KorokVisitor;
use codama_attributes::{
    Attribute, CodamaAttribute, CodamaDirective, EncodingDirective, FixedSizeDirective,
    SizePrefixDirective, TypeDirective,
};
use codama_errors::CodamaResult;
use codama_koroks::{KorokMut, KorokTrait};
use codama_nodes::{
    FixedSizeTypeNode, HasKind, NestedTypeLeaf, NestedTypeNode, NestedTypeNodeTrait, Node,
    RegisteredTypeNode, SizePrefixTypeNode, StringTypeNode, StructFieldTypeNode, TypeNode,
};
use codama_syn_helpers::extensions::ToTokensExtension;

#[derive(Default)]
pub struct ApplyCodamaTypeAttributesVisitor;

impl ApplyCodamaTypeAttributesVisitor {
    pub fn new() -> Self {
        Self
    }
}

impl KorokVisitor for ApplyCodamaTypeAttributesVisitor {
    fn visit_root(&mut self, korok: &mut codama_koroks::RootKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        Ok(())
    }

    fn visit_crate(&mut self, korok: &mut codama_koroks::CrateKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        apply_codama_attributes(korok.into())
    }

    fn visit_file_module(
        &mut self,
        korok: &mut codama_koroks::FileModuleKorok,
    ) -> CodamaResult<()> {
        self.visit_children(korok)?;
        apply_codama_attributes(korok.into())
    }

    fn visit_module(&mut self, korok: &mut codama_koroks::ModuleKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        apply_codama_attributes(korok.into())
    }

    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        apply_codama_attributes(korok.into())
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        apply_codama_attributes(korok.into())
    }

    fn visit_unsupported_item(
        &mut self,
        korok: &mut codama_koroks::UnsupportedItemKorok,
    ) -> CodamaResult<()> {
        self.visit_children(korok)?;
        apply_codama_attributes(korok.into())
    }

    fn visit_enum_variant(
        &mut self,
        korok: &mut codama_koroks::EnumVariantKorok,
    ) -> CodamaResult<()> {
        self.visit_children(korok)?;
        apply_codama_attributes(korok.into())
    }

    fn visit_field(&mut self, korok: &mut codama_koroks::FieldKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        apply_codama_attributes(korok.into())
    }
}

struct ApplyAttributeInput<'a, 'b> {
    node: Option<Node>,
    attribute: &'b CodamaAttribute<'b>,
    korok: &'b KorokMut<'a, 'b>,
}

/// Apply codama attributes to the node from the bottom up.
fn apply_codama_attributes(mut korok: KorokMut) -> CodamaResult<()> {
    let Some(attributes) = korok.attributes() else {
        return Ok(());
    };

    let node = attributes
        .iter()
        .filter_map(|attribute| match attribute {
            Attribute::Codama(attribute) => Some(attribute),
            _ => None,
        })
        .fold(
            Ok(korok.node().clone()),
            |current_node, attribute| match current_node {
                Ok(current_node) => apply_codama_attribute(ApplyAttributeInput {
                    node: current_node,
                    attribute,
                    korok: &korok,
                }),
                Err(e) => Err(e),
            },
        )?;

    korok.set_node(node);
    Ok(())
}

fn apply_codama_attribute(input: ApplyAttributeInput) -> CodamaResult<Option<Node>> {
    match &input.attribute.directive {
        CodamaDirective::Type(directive) => apply_type_directive(directive, input),
        CodamaDirective::Encoding(directive) => apply_encoding_directive(directive, input),
        CodamaDirective::FixedSize(directive) => apply_fixed_size_directive(directive, input),
        CodamaDirective::SizePrefix(directive) => apply_size_prefix_directive(directive, input),
    }
}

fn apply_type_directive(
    directive: &TypeDirective,
    input: ApplyAttributeInput,
) -> CodamaResult<Option<Node>> {
    let node = directive.node.clone();
    match input.korok {
        // If the `type` directive is applied to a named field then
        // we need to wrap the provided node in a `StructFieldTypeNode`.
        KorokMut::Field(korok) => match (TypeNode::try_from(node.clone()).ok(), &korok.ast.ident) {
            (Some(type_node), Some(ident)) => Ok(Some(
                StructFieldTypeNode::new(ident.to_string(), type_node).into(),
            )),
            _ => Ok(Some(node.into())),
        },
        _ => Ok(Some(node.into())),
    }
}

fn apply_encoding_directive(
    directive: &EncodingDirective,
    input: ApplyAttributeInput,
) -> CodamaResult<Option<Node>> {
    update_nested_type_node(input, |type_node| match type_node {
        TypeNode::String(_) => StringTypeNode::new(directive.encoding).into(),
        node => {
            // TODO: Throw error?
            node
        }
    })
}

fn apply_fixed_size_directive(
    directive: &FixedSizeDirective,
    input: ApplyAttributeInput,
) -> CodamaResult<Option<Node>> {
    update_type_node(input, |node| match node {
        TypeNode::FixedSize(node) => FixedSizeTypeNode::new(*node.r#type, directive.size).into(),
        TypeNode::SizePrefix(node) => FixedSizeTypeNode::new(*node.r#type, directive.size).into(),
        node => FixedSizeTypeNode::new(node, directive.size).into(),
    })
}

fn apply_size_prefix_directive(
    directive: &SizePrefixDirective,
    input: ApplyAttributeInput,
) -> CodamaResult<Option<Node>> {
    let prefix = directive.prefix.clone();
    update_type_node(input, |node| match node {
        TypeNode::FixedSize(node) => SizePrefixTypeNode::new(*node.r#type, prefix).into(),
        TypeNode::SizePrefix(node) => SizePrefixTypeNode::new(*node.r#type, prefix).into(),
        node => SizePrefixTypeNode::new(node, prefix).into(),
    })
}

fn update_nested_type_node(
    input: ApplyAttributeInput,
    update: impl FnOnce(TypeNode) -> TypeNode,
) -> CodamaResult<Option<Node>> {
    update_type_node(input, |type_node| {
        match NestedTypeNode::<NestedTypeLeaf>::try_from(type_node.clone()) {
            Ok(nested) => nested
                // Note that here we end up with a `NestedTypeLeaf` value...
                .map_nested_type_node(|leaf| NestedTypeLeaf(update(leaf.0)))
                // ...but here the `NestedTypeNode` value is unwrapped into a `TypeNode`.
                .into(),
            _ => unreachable!("NestedTypeLeaf can always be created from a TypeNode"),
        }
    })
}

/// Updates the type node within a given `Node` using the provided function.
/// If the `Node` is a `StructFieldTypeNode`, its type node will be updated; otherwise,
/// the function will attempt to update the type node of the `Node` itself.
fn update_type_node(
    input: ApplyAttributeInput,
    update: impl FnOnce(TypeNode) -> TypeNode,
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
        field.r#type = update(field.r#type);
        return Ok(Some(field.into()));
    };

    match TypeNode::try_from(node.clone()) {
        Ok(type_node) => Ok(Some(update(type_node).into())),
        Err(_) => {
            return Err(input
                .attribute
                .ast
                .error(format!(
                    "Cannot apply attribute `#[codama({})]` on a node of kind `{}`",
                    input.attribute.directive.name(),
                    node.kind()
                ))
                .into());
        }
    }
}
