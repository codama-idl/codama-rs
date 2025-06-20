use crate::KorokVisitable;
use codama_errors::{CodamaResult, IteratorCombineErrors};

pub trait KorokVisitor {
    fn visit_children(&mut self, korok: &mut dyn KorokVisitable) -> CodamaResult<()>
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

    fn visit_root(&mut self, korok: &mut codama_koroks::RootKorok) -> CodamaResult<()> {
        korok
            .crates
            .iter_mut()
            .map(|crate_korok| self.visit_crate(crate_korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_crate(&mut self, korok: &mut codama_koroks::CrateKorok) -> CodamaResult<()> {
        korok
            .items
            .iter_mut()
            .map(|item_korok| self.visit_item(item_korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_item(&mut self, korok: &mut codama_koroks::ItemKorok) -> CodamaResult<()> {
        match korok {
            codama_koroks::ItemKorok::FileModule(korok) => self.visit_file_module(korok),
            codama_koroks::ItemKorok::Module(korok) => self.visit_module(korok),
            codama_koroks::ItemKorok::Struct(korok) => self.visit_struct(korok),
            codama_koroks::ItemKorok::Enum(korok) => self.visit_enum(korok),
            codama_koroks::ItemKorok::Const(korok) => self.visit_const(korok),
            codama_koroks::ItemKorok::Unsupported(korok) => self.visit_unsupported_item(korok),
        }
    }

    fn visit_file_module(
        &mut self,
        korok: &mut codama_koroks::FileModuleKorok,
    ) -> CodamaResult<()> {
        korok
            .items
            .iter_mut()
            .map(|item_korok| self.visit_item(item_korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_module(&mut self, korok: &mut codama_koroks::ModuleKorok) -> CodamaResult<()> {
        korok
            .items
            .iter_mut()
            .map(|item_korok| self.visit_item(item_korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> CodamaResult<()> {
        self.visit_fields(&mut korok.fields)
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
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
    ) -> CodamaResult<()> {
        self.visit_fields(&mut korok.fields)
    }

    fn visit_unsupported_item(
        &mut self,
        _korok: &mut codama_koroks::UnsupportedItemKorok,
    ) -> CodamaResult<()> {
        Ok(())
    }

    fn visit_fields(&mut self, korok: &mut codama_koroks::FieldsKorok) -> CodamaResult<()> {
        korok
            .all
            .iter_mut()
            .map(|field_korok| self.visit_field(field_korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_field(&mut self, _korok: &mut codama_koroks::FieldKorok) -> CodamaResult<()> {
        Ok(())
    }

    fn visit_const(&mut self, _korok: &mut codama_koroks::ConstKorok) -> CodamaResult<()> {
        Ok(())
    }
}
