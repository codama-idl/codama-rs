use crate::KorokVisitor;
use codama_errors::CodamaResult;
use codama_koroks::KorokMut;

/// Use the same callback function on all koroks visited.
pub struct UniformVisitor {
    pub callback: fn(korok: KorokMut, visitor: &mut Self) -> CodamaResult<()>,
}

impl UniformVisitor {
    pub fn new(callback: fn(korok: KorokMut, visitor: &mut Self) -> CodamaResult<()>) -> Self {
        Self { callback }
    }
}

impl KorokVisitor for UniformVisitor {
    fn visit_root(&mut self, korok: &mut codama_koroks::RootKorok) -> CodamaResult<()> {
        (self.callback)(korok.into(), self)
    }

    fn visit_crate(&mut self, korok: &mut codama_koroks::CrateKorok) -> CodamaResult<()> {
        (self.callback)(korok.into(), self)
    }

    fn visit_item(&mut self, korok: &mut codama_koroks::ItemKorok) -> CodamaResult<()> {
        (self.callback)(korok.into(), self)
    }

    fn visit_file_module(
        &mut self,
        korok: &mut codama_koroks::FileModuleKorok,
    ) -> CodamaResult<()> {
        (self.callback)(korok.into(), self)
    }

    fn visit_module(&mut self, korok: &mut codama_koroks::ModuleKorok) -> CodamaResult<()> {
        (self.callback)(korok.into(), self)
    }

    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> CodamaResult<()> {
        (self.callback)(korok.into(), self)
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
        (self.callback)(korok.into(), self)
    }

    fn visit_enum_variant(
        &mut self,
        korok: &mut codama_koroks::EnumVariantKorok,
    ) -> CodamaResult<()> {
        (self.callback)(korok.into(), self)
    }

    fn visit_const(&mut self, korok: &mut codama_koroks::ConstKorok) -> CodamaResult<()> {
        (self.callback)(korok.into(), self)
    }

    fn visit_unsupported_item(
        &mut self,
        korok: &mut codama_koroks::UnsupportedItemKorok,
    ) -> CodamaResult<()> {
        (self.callback)(korok.into(), self)
    }

    fn visit_impl_item(&mut self, korok: &mut codama_koroks::ImplItemKorok) -> CodamaResult<()> {
        (self.callback)(korok.into(), self)
    }

    fn visit_unsupported_impl_item(
        &mut self,
        korok: &mut codama_koroks::UnsupportedImplItemKorok,
    ) -> CodamaResult<()> {
        (self.callback)(korok.into(), self)
    }

    fn visit_field(&mut self, korok: &mut codama_koroks::FieldKorok) -> CodamaResult<()> {
        (self.callback)(korok.into(), self)
    }
}
