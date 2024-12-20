use crate::KorokVisitor;
use codama_attributes::{
    Attribute, CodamaAttribute, CodamaDirective, EncodingDirective, FixedSizeDirective,
    TypeDirective,
};
use codama_koroks::{KorokMut, KorokTrait};
use codama_nodes::{
    FixedSizeTypeNode, NestedTypeLeaf, NestedTypeNode, NestedTypeNodeTrait, Node,
    RegisteredTypeNode, StringTypeNode, StructFieldTypeNode, TypeNode,
};

#[derive(Default)]
pub struct ApplyCodamaTypeAttributesVisitor;

impl ApplyCodamaTypeAttributesVisitor {
    pub fn new() -> Self {
        Self
    }
}

impl KorokVisitor for ApplyCodamaTypeAttributesVisitor {
    fn visit_root(&mut self, korok: &mut codama_koroks::RootKorok) {
        self.visit_children(korok);
    }

    fn visit_crate(&mut self, korok: &mut codama_koroks::CrateKorok) {
        self.visit_children(korok);
        apply_codama_attributes(korok.into());
    }

    fn visit_file_module(&mut self, korok: &mut codama_koroks::FileModuleKorok) {
        self.visit_children(korok);
        apply_codama_attributes(korok.into());
    }

    fn visit_module(&mut self, korok: &mut codama_koroks::ModuleKorok) {
        self.visit_children(korok);
        apply_codama_attributes(korok.into());
    }

    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) {
        self.visit_children(korok);
        apply_codama_attributes(korok.into());
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) {
        self.visit_children(korok);
        apply_codama_attributes(korok.into());
    }

    fn visit_unsupported_item(&mut self, korok: &mut codama_koroks::UnsupportedItemKorok) {
        self.visit_children(korok);
        apply_codama_attributes(korok.into());
    }

    fn visit_enum_variant(&mut self, korok: &mut codama_koroks::EnumVariantKorok) {
        self.visit_children(korok);
        apply_codama_attributes(korok.into());
    }

    fn visit_field(&mut self, korok: &mut codama_koroks::FieldKorok) {
        self.visit_children(korok);
        apply_codama_attributes(korok.into());
    }
}

/// Apply codama attributes to the node from the bottom up.
fn apply_codama_attributes(mut korok: KorokMut) {
    let Some(attributes) = korok.attributes() else {
        return;
    };

    let node = attributes
        .iter()
        .filter_map(|attribute| match attribute {
            Attribute::Codama(attribute) => Some(attribute),
            _ => None,
        })
        .fold(korok.node().clone(), |current_node, attribute| {
            apply_codama_attribute(current_node, attribute, &korok)
        });

    korok.set_node(node);
}

fn apply_codama_attribute(
    node: Option<Node>,
    attribute: &CodamaAttribute,
    korok: &KorokMut,
) -> Option<Node> {
    match &attribute.directive {
        CodamaDirective::Type(directive) => apply_type_directive(directive, korok),
        CodamaDirective::Encoding(directive) => apply_encoding_directive(directive, node),
        CodamaDirective::FixedSize(directive) => apply_fixed_size_directive(directive, node),
        _ => node,
    }
}

fn apply_type_directive(directive: &TypeDirective, korok: &KorokMut) -> Option<Node> {
    let node = directive.node.clone();
    match korok {
        // If the `type` directive is applied to a named field then
        // we need to wrap the provided node in a `StructFieldTypeNode`.
        KorokMut::Field(korok) => match (TypeNode::try_from(node.clone()).ok(), &korok.ast.ident) {
            (Some(type_node), Some(ident)) => {
                Some(StructFieldTypeNode::new(ident.to_string(), type_node).into())
            }
            _ => Some(node.into()),
        },
        _ => Some(node.into()),
    }
}

fn apply_encoding_directive(directive: &EncodingDirective, node: Option<Node>) -> Option<Node> {
    update_nested_type_node(node, |type_node| match type_node {
        TypeNode::String(_) => StringTypeNode::new(directive.encoding).into(),
        node => {
            // TODO: Throw error?
            node
        }
    })
}

fn apply_fixed_size_directive(directive: &FixedSizeDirective, node: Option<Node>) -> Option<Node> {
    update_type_node(node, |node| match node {
        TypeNode::FixedSize(node) => FixedSizeTypeNode::new(*node.r#type, directive.size).into(),
        TypeNode::SizePrefix(node) => FixedSizeTypeNode::new(*node.r#type, directive.size).into(),
        node => FixedSizeTypeNode::new(node, directive.size).into(),
    })
}

fn update_nested_type_node(
    node: Option<Node>,
    update: impl FnOnce(TypeNode) -> TypeNode,
) -> Option<Node> {
    update_type_node(
        node,
        |type_node| match NestedTypeNode::<NestedTypeLeaf>::try_from(type_node.clone()) {
            Ok(nested) => nested
                // Note that here we end up with a `NestedTypeLeaf` value...
                .map_nested_type_node(|leaf| NestedTypeLeaf(update(leaf.0)))
                // ...but here the `NestedTypeNode` value is unwrapped into a `TypeNode`.
                .into(),
            _ => unreachable!("NestedTypeLeaf can always be created from a TypeNode"),
        },
    )
}

/// Updates the type node within a given `Node` using the provided function.
/// If the `Node` is a `StructFieldTypeNode`, its type node will be updated; otherwise,
/// the function will attempt to update the type node of the `Node` itself.
fn update_type_node(node: Option<Node>, update: impl FnOnce(TypeNode) -> TypeNode) -> Option<Node> {
    let Some(node) = node else {
        // TODO: Throw error?
        return None;
    };

    if let Node::Type(RegisteredTypeNode::StructField(mut field)) = node {
        field.r#type = update(field.r#type);
        return Some(field.into());
    };

    match TypeNode::try_from(node.clone()) {
        Ok(type_node) => Some(update(type_node).into()),
        Err(_) => {
            // TODO: Throw error?
            Some(node)
        }
    }
}
