use crate::koroks::{
    CrateKorok, EnumKorok, EnumVariantKorok, FieldKorok, FileModuleKorok, ItemKorok, ModuleKorok,
    RootKorok, StructKorok, UnsupportedItemKorok,
};

pub trait KorokVisitor {
    fn visit_root(&mut self, korok: &RootKorok) {
        for crate_korok in &korok.crates {
            self.visit_crate(crate_korok);
        }
    }

    fn visit_crate(&mut self, korok: &CrateKorok) {
        for item_korok in &korok.items {
            self.visit_item(item_korok);
        }
    }

    fn visit_item(&mut self, korok: &ItemKorok) {
        match korok {
            ItemKorok::FileModule(korok) => self.visit_file_module(korok),
            ItemKorok::Module(korok) => self.visit_module(korok),
            ItemKorok::Struct(korok) => self.visit_struct(korok),
            ItemKorok::Enum(korok) => self.visit_enum(korok),
            ItemKorok::Unsupported(korok) => self.visit_unsupported_item(korok),
        }
    }

    fn visit_file_module(&mut self, korok: &FileModuleKorok) {
        for item_korok in &korok.items {
            self.visit_item(item_korok);
        }
    }

    fn visit_module(&mut self, korok: &ModuleKorok) {
        for item_korok in &korok.items {
            self.visit_item(item_korok);
        }
    }

    fn visit_struct(&mut self, korok: &StructKorok) {
        for field_korok in &korok.fields {
            self.visit_field(field_korok);
        }
    }

    fn visit_field(&mut self, _korok: &FieldKorok) {
        //
    }

    fn visit_enum(&mut self, korok: &EnumKorok) {
        for variant_korok in &korok.variants {
            self.visit_enum_variant(variant_korok);
        }
    }

    fn visit_enum_variant(&mut self, korok: &EnumVariantKorok) {
        for field_korok in &korok.fields {
            self.visit_field(field_korok);
        }
    }

    fn visit_unsupported_item(&mut self, _korok: &UnsupportedItemKorok) {
        //
    }
}
