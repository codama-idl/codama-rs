use crate::KorokVisitor;
use codama_koroks::KorokMut;

/// Use the same callback function on all koroks visited.
/// TODO: Rename UniformVisitor?
pub struct MapVisitor {
    pub callback: fn(korok: KorokMut, visitor: &mut Self),
}

impl MapVisitor {
    pub fn new(callback: fn(korok: KorokMut, visitor: &mut Self)) -> Self {
        Self { callback }
    }
}

impl KorokVisitor for MapVisitor {
    fn visit_root(&mut self, korok: &mut codama_koroks::RootKorok) {
        (self.callback)(korok.into(), self);
    }

    fn visit_crate(&mut self, korok: &mut codama_koroks::CrateKorok) {
        (self.callback)(korok.into(), self);
    }

    fn visit_item(&mut self, korok: &mut codama_koroks::ItemKorok) {
        (self.callback)(korok.into(), self);
    }

    fn visit_file_module(&mut self, korok: &mut codama_koroks::FileModuleKorok) {
        (self.callback)(korok.into(), self);
    }

    fn visit_module(&mut self, korok: &mut codama_koroks::ModuleKorok) {
        (self.callback)(korok.into(), self);
    }

    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) {
        (self.callback)(korok.into(), self);
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) {
        (self.callback)(korok.into(), self);
    }

    fn visit_enum_variant(&mut self, korok: &mut codama_koroks::EnumVariantKorok) {
        (self.callback)(korok.into(), self);
    }

    fn visit_unsupported_item(&mut self, korok: &mut codama_koroks::UnsupportedItemKorok) {
        (self.callback)(korok.into(), self);
    }

    fn visit_fields(&mut self, korok: &mut codama_koroks::FieldsKorok) {
        (self.callback)(korok.into(), self);
    }

    fn visit_field(&mut self, korok: &mut codama_koroks::FieldKorok) {
        (self.callback)(korok.into(), self);
    }

    fn visit_type(&mut self, korok: &mut codama_koroks::TypeKorok) {
        (self.callback)(korok.into(), self);
    }
}
