use crate::visitor::KorokVisitor;

pub trait KorokVisitable {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor);
    fn get_children(&mut self) -> Vec<&mut dyn KorokVisitable>;
}

impl KorokVisitable for codama_koroks::KorokMut<'_, '_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) {
        match self {
            Self::Crate(k) => k.accept(visitor),
            Self::Enum(k) => k.accept(visitor),
            Self::EnumVariant(k) => k.accept(visitor),
            Self::Field(k) => k.accept(visitor),
            Self::Fields(k) => k.accept(visitor),
            Self::FileModule(k) => k.accept(visitor),
            Self::Item(k) => k.accept(visitor),
            Self::Module(k) => k.accept(visitor),
            Self::Root(k) => k.accept(visitor),
            Self::Struct(k) => k.accept(visitor),
            Self::Type(k) => k.accept(visitor),
            Self::UnsupportedItem(k) => k.accept(visitor),
        }
    }

    fn get_children(&mut self) -> Vec<&mut dyn KorokVisitable> {
        match self {
            Self::Crate(k) => k.get_children(),
            Self::Enum(k) => k.get_children(),
            Self::EnumVariant(k) => k.get_children(),
            Self::Field(k) => k.get_children(),
            Self::Fields(k) => k.get_children(),
            Self::FileModule(k) => k.get_children(),
            Self::Item(k) => k.get_children(),
            Self::Module(k) => k.get_children(),
            Self::Root(k) => k.get_children(),
            Self::Struct(k) => k.get_children(),
            Self::Type(k) => k.get_children(),
            Self::UnsupportedItem(k) => k.get_children(),
        }
    }
}

impl KorokVisitable for codama_koroks::RootKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_root(self);
    }
    fn get_children(&mut self) -> Vec<&mut dyn KorokVisitable> {
        self.crates
            .iter_mut()
            .map(|c| c as &mut dyn KorokVisitable)
            .collect()
    }
}

impl KorokVisitable for codama_koroks::CrateKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_crate(self);
    }
    fn get_children(&mut self) -> Vec<&mut dyn KorokVisitable> {
        self.items
            .iter_mut()
            .map(|i| i as &mut dyn KorokVisitable)
            .collect()
    }
}

impl KorokVisitable for codama_koroks::ItemKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_item(self);
    }
    fn get_children(&mut self) -> Vec<&mut dyn KorokVisitable> {
        match self {
            codama_koroks::ItemKorok::FileModule(k) => vec![k as &mut dyn KorokVisitable],
            codama_koroks::ItemKorok::Module(k) => vec![k as &mut dyn KorokVisitable],
            codama_koroks::ItemKorok::Struct(k) => vec![k as &mut dyn KorokVisitable],
            codama_koroks::ItemKorok::Enum(k) => vec![k as &mut dyn KorokVisitable],
            codama_koroks::ItemKorok::Unsupported(k) => vec![k as &mut dyn KorokVisitable],
        }
    }
}

impl KorokVisitable for codama_koroks::FileModuleKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_file_module(self);
    }
    fn get_children(&mut self) -> Vec<&mut dyn KorokVisitable> {
        self.items
            .iter_mut()
            .map(|i| i as &mut dyn KorokVisitable)
            .collect()
    }
}

impl KorokVisitable for codama_koroks::ModuleKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_module(self);
    }
    fn get_children(&mut self) -> Vec<&mut dyn KorokVisitable> {
        self.items
            .iter_mut()
            .map(|i| i as &mut dyn KorokVisitable)
            .collect()
    }
}

impl KorokVisitable for codama_koroks::StructKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_struct(self);
    }
    fn get_children(&mut self) -> Vec<&mut dyn KorokVisitable> {
        vec![&mut self.fields as &mut dyn KorokVisitable]
    }
}

impl KorokVisitable for codama_koroks::FieldsKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_fields(self);
    }
    fn get_children(&mut self) -> Vec<&mut dyn KorokVisitable> {
        self.all
            .iter_mut()
            .map(|f| f as &mut dyn KorokVisitable)
            .collect()
    }
}

impl KorokVisitable for codama_koroks::FieldKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_field(self);
    }
    fn get_children(&mut self) -> Vec<&mut dyn KorokVisitable> {
        vec![&mut self.r#type as &mut dyn KorokVisitable]
    }
}

impl KorokVisitable for codama_koroks::TypeKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_type(self);
    }
    fn get_children(&mut self) -> Vec<&mut dyn KorokVisitable> {
        Vec::new()
    }
}

impl KorokVisitable for codama_koroks::EnumKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_enum(self);
    }
    fn get_children(&mut self) -> Vec<&mut dyn KorokVisitable> {
        self.variants
            .iter_mut()
            .map(|v| v as &mut dyn KorokVisitable)
            .collect()
    }
}

impl KorokVisitable for codama_koroks::EnumVariantKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_enum_variant(self);
    }
    fn get_children(&mut self) -> Vec<&mut dyn KorokVisitable> {
        vec![&mut self.fields as &mut dyn KorokVisitable]
    }
}

impl KorokVisitable for codama_koroks::UnsupportedItemKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_unsupported_item(self);
    }
    fn get_children(&mut self) -> Vec<&mut dyn KorokVisitable> {
        Vec::new()
    }
}
