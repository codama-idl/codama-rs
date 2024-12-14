use crate::KorokVisitor;
use codama_attributes::{ApplyToNode, Attribute};
use codama_koroks::{KorokMut, KorokTrait};

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
            attribute.apply(current_node)
        });

    korok.set_node(node);
}
