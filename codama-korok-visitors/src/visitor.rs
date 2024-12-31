use crate::KorokVisitable;
use codama_errors::IteratorCombineErrors;

pub trait KorokVisitor {
    fn visit_children(&mut self, korok: &mut dyn KorokVisitable) -> syn::Result<()>
    where
        Self: Sized,
    {
        korok
            .get_children()
            .into_iter()
            .map(|child| child.accept(self))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_root(&mut self, korok: &mut codama_koroks::RootKorok) -> syn::Result<()> {
        korok
            .crates
            .iter_mut()
            .map(|crate_korok| self.visit_crate(crate_korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_crate(&mut self, korok: &mut codama_koroks::CrateKorok) -> syn::Result<()> {
        korok
            .items
            .iter_mut()
            .map(|item_korok| self.visit_item(item_korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_item(&mut self, korok: &mut codama_koroks::ItemKorok) -> syn::Result<()> {
        match korok {
            codama_koroks::ItemKorok::FileModule(korok) => self.visit_file_module(korok),
            codama_koroks::ItemKorok::Module(korok) => self.visit_module(korok),
            codama_koroks::ItemKorok::Struct(korok) => self.visit_struct(korok),
            codama_koroks::ItemKorok::Enum(korok) => self.visit_enum(korok),
            codama_koroks::ItemKorok::Unsupported(korok) => self.visit_unsupported_item(korok),
        }
    }

    fn visit_file_module(&mut self, korok: &mut codama_koroks::FileModuleKorok) -> syn::Result<()> {
        korok
            .items
            .iter_mut()
            .map(|item_korok| self.visit_item(item_korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_module(&mut self, korok: &mut codama_koroks::ModuleKorok) -> syn::Result<()> {
        korok
            .items
            .iter_mut()
            .map(|item_korok| self.visit_item(item_korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> syn::Result<()> {
        self.visit_fields(&mut korok.fields)
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> syn::Result<()> {
        korok
            .variants
            .iter_mut()
            .map(|variant_korok| self.visit_enum_variant(variant_korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_enum_variant(
        &mut self,
        korok: &mut codama_koroks::EnumVariantKorok,
    ) -> syn::Result<()> {
        self.visit_fields(&mut korok.fields)
    }

    fn visit_unsupported_item(
        &mut self,
        _korok: &mut codama_koroks::UnsupportedItemKorok,
    ) -> syn::Result<()> {
        Ok(())
    }

    fn visit_fields(&mut self, korok: &mut codama_koroks::FieldsKorok) -> syn::Result<()> {
        korok
            .all
            .iter_mut()
            .map(|field_korok| self.visit_field(field_korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_field(&mut self, _korok: &mut codama_koroks::FieldKorok) -> syn::Result<()> {
        Ok(())
    }
}
