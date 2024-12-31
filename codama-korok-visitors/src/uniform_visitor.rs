use crate::KorokVisitor;
use codama_koroks::KorokMut;

/// Use the same callback function on all koroks visited.
pub struct UniformVisitor {
    pub callback: fn(korok: KorokMut, visitor: &mut Self) -> syn::Result<()>,
}

impl UniformVisitor {
    pub fn new(callback: fn(korok: KorokMut, visitor: &mut Self) -> syn::Result<()>) -> Self {
        Self { callback }
    }
}

impl KorokVisitor for UniformVisitor {
    fn visit_root(&mut self, korok: &mut codama_koroks::RootKorok) -> syn::Result<()> {
        (self.callback)(korok.into(), self)
    }

    fn visit_crate(&mut self, korok: &mut codama_koroks::CrateKorok) -> syn::Result<()> {
        (self.callback)(korok.into(), self)
    }

    fn visit_item(&mut self, korok: &mut codama_koroks::ItemKorok) -> syn::Result<()> {
        (self.callback)(korok.into(), self)
    }

    fn visit_file_module(&mut self, korok: &mut codama_koroks::FileModuleKorok) -> syn::Result<()> {
        (self.callback)(korok.into(), self)
    }

    fn visit_module(&mut self, korok: &mut codama_koroks::ModuleKorok) -> syn::Result<()> {
        (self.callback)(korok.into(), self)
    }

    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> syn::Result<()> {
        (self.callback)(korok.into(), self)
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> syn::Result<()> {
        (self.callback)(korok.into(), self)
    }

    fn visit_enum_variant(
        &mut self,
        korok: &mut codama_koroks::EnumVariantKorok,
    ) -> syn::Result<()> {
        (self.callback)(korok.into(), self)
    }

    fn visit_unsupported_item(
        &mut self,
        korok: &mut codama_koroks::UnsupportedItemKorok,
    ) -> syn::Result<()> {
        (self.callback)(korok.into(), self)
    }

    fn visit_fields(&mut self, korok: &mut codama_koroks::FieldsKorok) -> syn::Result<()> {
        (self.callback)(korok.into(), self)
    }

    fn visit_field(&mut self, korok: &mut codama_koroks::FieldKorok) -> syn::Result<()> {
        (self.callback)(korok.into(), self)
    }
}
