use crate::KorokVisitor;
use codama_koroks::Korok;

/// Use the same callback function on all koroks visited.
pub struct MapVisitor {
    pub callback: fn(korok: &mut dyn Korok) -> (),
}

impl MapVisitor {
    pub fn new(callback: fn(korok: &mut dyn Korok) -> ()) -> Self {
        Self { callback }
    }
}

impl KorokVisitor for MapVisitor {
    fn visit_root(&mut self, korok: &mut codama_koroks::RootKorok) {
        self.visit_children(korok);
        (self.callback)(korok);
    }

    fn visit_crate(&mut self, korok: &mut codama_koroks::CrateKorok) {
        self.visit_children(korok);
        (self.callback)(korok);
    }

    fn visit_item(&mut self, korok: &mut codama_koroks::ItemKorok) {
        self.visit_children(korok);
        (self.callback)(korok);
    }

    fn visit_file_module(&mut self, korok: &mut codama_koroks::FileModuleKorok) {
        self.visit_children(korok);
        (self.callback)(korok);
    }

    fn visit_module(&mut self, korok: &mut codama_koroks::ModuleKorok) {
        self.visit_children(korok);
        (self.callback)(korok);
    }

    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) {
        self.visit_children(korok);
        (self.callback)(korok);
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) {
        self.visit_children(korok);
        (self.callback)(korok);
    }

    fn visit_enum_variant(&mut self, korok: &mut codama_koroks::EnumVariantKorok) {
        self.visit_children(korok);
        (self.callback)(korok);
    }

    fn visit_unsupported_item(&mut self, korok: &mut codama_koroks::UnsupportedItemKorok) {
        self.visit_children(korok);
        (self.callback)(korok);
    }

    fn visit_fields(&mut self, korok: &mut codama_koroks::FieldsKorok) {
        self.visit_children(korok);
        (self.callback)(korok);
    }

    fn visit_field(&mut self, korok: &mut codama_koroks::FieldKorok) {
        self.visit_children(korok);
        (self.callback)(korok);
    }

    fn visit_type(&mut self, korok: &mut codama_koroks::TypeKorok) {
        self.visit_children(korok);
        (self.callback)(korok);
    }
}
