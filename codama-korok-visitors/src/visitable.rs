use crate::visitor::KorokVisitor;

pub trait KorokVisitable {
    fn accept(&self, visitor: &mut dyn KorokVisitor);
}

impl KorokVisitable for codama_koroks::RootKorok<'_> {
    fn accept(&self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_root(self);
    }
}

impl KorokVisitable for codama_koroks::CrateKorok<'_> {
    fn accept(&self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_crate(self);
    }
}

impl KorokVisitable for codama_koroks::ItemKorok<'_> {
    fn accept(&self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_item(self);
    }
}

impl KorokVisitable for codama_koroks::FileModuleKorok<'_> {
    fn accept(&self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_file_module(self);
    }
}

impl KorokVisitable for codama_koroks::ModuleKorok<'_> {
    fn accept(&self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_module(self);
    }
}

impl KorokVisitable for codama_koroks::StructKorok<'_> {
    fn accept(&self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_struct(self);
    }
}

impl KorokVisitable for codama_koroks::FieldKorok<'_> {
    fn accept(&self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_field(self);
    }
}

impl KorokVisitable for codama_koroks::EnumKorok<'_> {
    fn accept(&self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_enum(self);
    }
}

impl KorokVisitable for codama_koroks::EnumVariantKorok<'_> {
    fn accept(&self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_enum_variant(self);
    }
}

impl KorokVisitable for codama_koroks::UnsupportedItemKorok<'_> {
    fn accept(&self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_unsupported_item(self);
    }
}
