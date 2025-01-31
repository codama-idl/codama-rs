use codama_errors::{CodamaResult, IteratorCombineErrors};

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

    /// Adds a new visitor to the composition.
    pub fn with<T: KorokVisitor + 'a>(mut self, visitor: T) -> Self {
        self.visitors.push(Box::new(visitor));
        self
    }
}

impl KorokVisitor for ComposeVisitor<'_> {
    fn visit_root(&mut self, korok: &mut codama_koroks::RootKorok) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|v| v.visit_root(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_crate(&mut self, korok: &mut codama_koroks::CrateKorok) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|v| v.visit_crate(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_item(&mut self, korok: &mut codama_koroks::ItemKorok) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|v| v.visit_item(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_file_module(
        &mut self,
        korok: &mut codama_koroks::FileModuleKorok,
    ) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|v| v.visit_file_module(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_module(&mut self, korok: &mut codama_koroks::ModuleKorok) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|v| v.visit_module(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|v| v.visit_struct(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|v| v.visit_enum(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_enum_variant(
        &mut self,
        korok: &mut codama_koroks::EnumVariantKorok,
    ) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|v| v.visit_enum_variant(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_unsupported_item(
        &mut self,
        korok: &mut codama_koroks::UnsupportedItemKorok,
    ) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|v| v.visit_unsupported_item(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_fields(&mut self, korok: &mut codama_koroks::FieldsKorok) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|v| v.visit_fields(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_field(&mut self, korok: &mut codama_koroks::FieldKorok) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|v| v.visit_field(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }
}
