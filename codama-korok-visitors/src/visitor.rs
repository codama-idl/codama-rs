pub trait KorokVisitor {
    fn visit_root(&mut self, korok: &mut codama_koroks::RootKorok) {
        for crate_korok in &mut korok.crates {
            self.visit_crate(crate_korok);
        }
    }

    fn visit_crate(&mut self, korok: &mut codama_koroks::CrateKorok) {
        for item_korok in &mut korok.items {
            self.visit_item(item_korok);
        }
    }

    fn visit_item(&mut self, korok: &mut codama_koroks::ItemKorok) {
        match korok {
            codama_koroks::ItemKorok::FileModule(korok) => self.visit_file_module(korok),
            codama_koroks::ItemKorok::Module(korok) => self.visit_module(korok),
            codama_koroks::ItemKorok::Struct(korok) => self.visit_struct(korok),
            codama_koroks::ItemKorok::Enum(korok) => self.visit_enum(korok),
            codama_koroks::ItemKorok::Unsupported(korok) => self.visit_unsupported_item(korok),
        }
    }

    fn visit_file_module(&mut self, korok: &mut codama_koroks::FileModuleKorok) {
        for item_korok in &mut korok.items {
            self.visit_item(item_korok);
        }
    }

    fn visit_module(&mut self, korok: &mut codama_koroks::ModuleKorok) {
        for item_korok in &mut korok.items {
            self.visit_item(item_korok);
        }
    }

    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) {
        self.visit_fields(&mut korok.fields);
    }

    fn visit_fields(&mut self, korok: &mut codama_koroks::FieldsKorok) {
        for field_korok in &mut korok.all {
            self.visit_field(field_korok);
        }
    }

    fn visit_field(&mut self, korok: &mut codama_koroks::FieldKorok) {
        self.visit_type(&mut korok.r#type);
    }

    fn visit_type(&mut self, _korok: &mut codama_koroks::TypeKorok) {
        //
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) {
        for variant_korok in &mut korok.variants {
            self.visit_enum_variant(variant_korok);
        }
    }

    fn visit_enum_variant(&mut self, korok: &mut codama_koroks::EnumVariantKorok) {
        self.visit_fields(&mut korok.fields);
    }

    fn visit_unsupported_item(&mut self, _korok: &mut codama_koroks::UnsupportedItemKorok) {
        //
    }
}
