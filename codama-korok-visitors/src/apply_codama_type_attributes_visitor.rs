use crate::KorokVisitor;
use codama_attributes::{
    Attribute, CodamaAttribute, CodamaDirective, EncodingDirective, TypeDirective,
};
use codama_koroks::{KorokMut, KorokTrait};
use codama_nodes::{
    NestedTypeNode, NestedTypeNodeTrait, Node, RegisteredTypeNode, StringTypeNode,
    StructFieldTypeNode, TypeNode,
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
    let Some(node) = node else {
        // TODO: Throw error
        return None;
    };
    let update_node = |_: StringTypeNode| StringTypeNode::new(directive.encoding);

    if let Ok(nested) = NestedTypeNode::<StringTypeNode>::try_from(node.clone()) {
        // TODO: Remove unwrap
        return Some(nested.map_nested_type_node(update_node).try_into().unwrap());
    };

    match node {
        Node::Type(RegisteredTypeNode::String(node)) => Some(update_node(node).into()),
        Node::Type(RegisteredTypeNode::StructField(mut field)) => {
            field.r#type = match field.r#type {
                TypeNode::String(node) => update_node(node).into(),
                node => node,
            };
            Some(field.into())
        }
        node => Some(node),
    }
}
