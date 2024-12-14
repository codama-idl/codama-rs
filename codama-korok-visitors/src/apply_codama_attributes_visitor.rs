use crate::KorokVisitor;
use codama_attributes::{Attribute, CodamaAttribute, CodamaDirective, NodeDirective};
use codama_koroks::{KorokMut, KorokTrait};
use codama_nodes::{Node, StructFieldTypeNode, TypeNode};

#[derive(Default)]
pub struct ApplyCodamaAttributesVisitor;

impl ApplyCodamaAttributesVisitor {
    pub fn new() -> Self {
        Self
    }
}

impl KorokVisitor for ApplyCodamaAttributesVisitor {
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
        .rev()
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
        CodamaDirective::Node(directive) => apply_node_directive(directive, korok),
        _ => node,
    }
}

fn apply_node_directive(directive: &NodeDirective, korok: &KorokMut) -> Option<Node> {
    let node = directive.node.clone();
    match korok {
        // If the `node` directive is applied to a named field and
        // the node is a type node (i.e. excluding `StructFieldTypeNodes`)
        // then we need to wrap the provided node in a `StructFieldTypeNode`.
        KorokMut::Field(korok) => match (TypeNode::try_from(node.clone()).ok(), &korok.ast.ident) {
            (Some(type_node), Some(ident)) => {
                Some(StructFieldTypeNode::new(ident.to_string(), type_node).into())
            }
            _ => Some(node),
        },
        _ => Some(node),
    }
}
