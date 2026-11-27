use crate::visitor::KorokVisitor;
use codama_errors::CodamaResult;

/// A korok that a [`KorokVisitor`] can visit through dynamic dispatch, which calls the visitor
/// method of its kind.
pub trait KorokVisitable {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) -> CodamaResult<()>;
}

impl KorokVisitable for codama_koroks::KorokMut<'_, '_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) -> CodamaResult<()> {
        match self {
            Self::Crate(k) => k.accept(visitor),
            Self::Enum(k) => k.accept(visitor),
            Self::EnumVariant(k) => k.accept(visitor),
            Self::Field(k) => k.accept(visitor),
            Self::FileModule(k) => k.accept(visitor),
            Self::Impl(k) => k.accept(visitor),
            Self::Item(k) => k.accept(visitor),
            Self::Module(k) => k.accept(visitor),
            Self::Root(k) => k.accept(visitor),
            Self::Struct(k) => k.accept(visitor),
            Self::Const(k) => k.accept(visitor),
            Self::TypeAlias(k) => k.accept(visitor),
            Self::UnsupportedItem(k) => k.accept(visitor),
            Self::ImplItem(k) => k.accept(visitor),
            Self::UnsupportedImplItem(k) => k.accept(visitor),
        }
    }
}

impl KorokVisitable for codama_koroks::RootKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) -> CodamaResult<()> {
        visitor.visit_root(self)
    }
}

impl KorokVisitable for codama_koroks::CrateKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) -> CodamaResult<()> {
        visitor.visit_crate(self)
    }
}

impl KorokVisitable for codama_koroks::ItemKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) -> CodamaResult<()> {
        visitor.visit_item(self)
    }
}

impl KorokVisitable for codama_koroks::FileModuleKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) -> CodamaResult<()> {
        visitor.visit_file_module(self)
    }
}

impl KorokVisitable for codama_koroks::ModuleKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) -> CodamaResult<()> {
        visitor.visit_module(self)
    }
}

impl KorokVisitable for codama_koroks::StructKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) -> CodamaResult<()> {
        visitor.visit_struct(self)
    }
}

impl KorokVisitable for codama_koroks::FieldKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) -> CodamaResult<()> {
        visitor.visit_field(self)
    }
}

impl KorokVisitable for codama_koroks::EnumKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) -> CodamaResult<()> {
        visitor.visit_enum(self)
    }
}

impl KorokVisitable for codama_koroks::EnumVariantKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) -> CodamaResult<()> {
        visitor.visit_enum_variant(self)
    }
}

impl KorokVisitable for codama_koroks::ImplKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) -> CodamaResult<()> {
        visitor.visit_impl(self)
    }
}

impl KorokVisitable for codama_koroks::ImplItemKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) -> CodamaResult<()> {
        visitor.visit_impl_item(self)
    }
}

impl KorokVisitable for codama_koroks::UnsupportedImplItemKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) -> CodamaResult<()> {
        visitor.visit_unsupported_impl_item(self)
    }
}

impl KorokVisitable for codama_koroks::ConstKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) -> CodamaResult<()> {
        visitor.visit_const(self)
    }
}

impl KorokVisitable for codama_koroks::TypeAliasKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) -> CodamaResult<()> {
        visitor.visit_type_alias(self)
    }
}

impl KorokVisitable for codama_koroks::UnsupportedItemKorok<'_> {
    fn accept(&mut self, visitor: &mut dyn KorokVisitor) -> CodamaResult<()> {
        visitor.visit_unsupported_item(self)
    }
}
