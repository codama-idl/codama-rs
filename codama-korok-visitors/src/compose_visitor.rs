use crate::KorokVisitor;

/// Compose multiple visitors into one.
#[derive(Default)]
pub struct ComposeVisitor<'a> {
    pub visitors: Vec<Box<dyn KorokVisitor + 'a>>,
}

impl<'a> ComposeVisitor<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a new visitor to the composition.
    pub fn add<T: KorokVisitor + 'a>(mut self, visitor: T) -> Self {
        self.visitors.push(Box::new(visitor));
        self
    }
}

impl KorokVisitor for ComposeVisitor<'_> {
    fn visit_root(&mut self, korok: &mut codama_koroks::RootKorok) {
        for visitor in &mut self.visitors {
            visitor.visit_root(korok);
        }
    }

    fn visit_crate(&mut self, korok: &mut codama_koroks::CrateKorok) {
        for visitor in &mut self.visitors {
            visitor.visit_crate(korok);
        }
    }

    fn visit_item(&mut self, korok: &mut codama_koroks::ItemKorok) {
        for visitor in &mut self.visitors {
            visitor.visit_item(korok);
        }
    }

    fn visit_file_module(&mut self, korok: &mut codama_koroks::FileModuleKorok) {
        for visitor in &mut self.visitors {
            visitor.visit_file_module(korok);
        }
    }

    fn visit_module(&mut self, korok: &mut codama_koroks::ModuleKorok) {
        for visitor in &mut self.visitors {
            visitor.visit_module(korok);
        }
    }

    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) {
        for visitor in &mut self.visitors {
            visitor.visit_struct(korok);
        }
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) {
        for visitor in &mut self.visitors {
            visitor.visit_enum(korok);
        }
    }

    fn visit_enum_variant(&mut self, korok: &mut codama_koroks::EnumVariantKorok) {
        for visitor in &mut self.visitors {
            visitor.visit_enum_variant(korok);
        }
    }

    fn visit_unsupported_item(&mut self, korok: &mut codama_koroks::UnsupportedItemKorok) {
        for visitor in &mut self.visitors {
            visitor.visit_unsupported_item(korok);
        }
    }

    fn visit_fields(&mut self, korok: &mut codama_koroks::FieldsKorok) {
        for visitor in &mut self.visitors {
            visitor.visit_fields(korok);
        }
    }

    fn visit_field(&mut self, korok: &mut codama_koroks::FieldKorok) {
        for visitor in &mut self.visitors {
            visitor.visit_field(korok);
        }
    }
}
