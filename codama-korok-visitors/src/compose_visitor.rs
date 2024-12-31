use codama_errors::IteratorCombineErrors;

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
    fn visit_root(&mut self, korok: &mut codama_koroks::RootKorok) -> syn::Result<()> {
        self.visitors
            .iter_mut()
            .map(|v| v.visit_root(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_crate(&mut self, korok: &mut codama_koroks::CrateKorok) -> syn::Result<()> {
        self.visitors
            .iter_mut()
            .map(|v| v.visit_crate(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_item(&mut self, korok: &mut codama_koroks::ItemKorok) -> syn::Result<()> {
        self.visitors
            .iter_mut()
            .map(|v| v.visit_item(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_file_module(&mut self, korok: &mut codama_koroks::FileModuleKorok) -> syn::Result<()> {
        self.visitors
            .iter_mut()
            .map(|v| v.visit_file_module(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_module(&mut self, korok: &mut codama_koroks::ModuleKorok) -> syn::Result<()> {
        self.visitors
            .iter_mut()
            .map(|v| v.visit_module(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> syn::Result<()> {
        self.visitors
            .iter_mut()
            .map(|v| v.visit_struct(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> syn::Result<()> {
        self.visitors
            .iter_mut()
            .map(|v| v.visit_enum(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_enum_variant(
        &mut self,
        korok: &mut codama_koroks::EnumVariantKorok,
    ) -> syn::Result<()> {
        self.visitors
            .iter_mut()
            .map(|v| v.visit_enum_variant(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_unsupported_item(
        &mut self,
        korok: &mut codama_koroks::UnsupportedItemKorok,
    ) -> syn::Result<()> {
        self.visitors
            .iter_mut()
            .map(|v| v.visit_unsupported_item(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_fields(&mut self, korok: &mut codama_koroks::FieldsKorok) -> syn::Result<()> {
        self.visitors
            .iter_mut()
            .map(|v| v.visit_fields(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_field(&mut self, korok: &mut codama_koroks::FieldKorok) -> syn::Result<()> {
        self.visitors
            .iter_mut()
            .map(|v| v.visit_field(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }
}
