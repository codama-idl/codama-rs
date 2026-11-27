use crate::KorokVisitor;
use codama_errors::{CodamaResult, IteratorCombineErrors};
use codama_koroks::{
    ConstKorok, CrateKorok, EnumKorok, EnumVariantKorok, FieldKorok, FileModuleKorok,
    ImplItemKorok, ImplKorok, ItemKorok, ModuleKorok, RootKorok, StructKorok, TypeAliasKorok,
    UnsupportedImplItemKorok, UnsupportedItemKorok,
};

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

/// Every method forwards to each visitor in turn, so the composition visits exactly what its
/// visitors visit and never walks on their behalf.
impl KorokVisitor for ComposeVisitor<'_> {
    fn visit_root(&mut self, korok: &mut RootKorok) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|visitor| visitor.visit_root(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_crate(&mut self, korok: &mut CrateKorok) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|visitor| visitor.visit_crate(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_item(&mut self, korok: &mut ItemKorok) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|visitor| visitor.visit_item(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_file_module(&mut self, korok: &mut FileModuleKorok) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|visitor| visitor.visit_file_module(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_module(&mut self, korok: &mut ModuleKorok) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|visitor| visitor.visit_module(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_struct(&mut self, korok: &mut StructKorok) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|visitor| visitor.visit_struct(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_enum(&mut self, korok: &mut EnumKorok) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|visitor| visitor.visit_enum(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_enum_variant(&mut self, korok: &mut EnumVariantKorok) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|visitor| visitor.visit_enum_variant(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_field(&mut self, korok: &mut FieldKorok) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|visitor| visitor.visit_field(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_impl(&mut self, korok: &mut ImplKorok) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|visitor| visitor.visit_impl(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_impl_item(&mut self, korok: &mut ImplItemKorok) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|visitor| visitor.visit_impl_item(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_const(&mut self, korok: &mut ConstKorok) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|visitor| visitor.visit_const(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_type_alias(&mut self, korok: &mut TypeAliasKorok) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|visitor| visitor.visit_type_alias(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_unsupported_item(&mut self, korok: &mut UnsupportedItemKorok) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|visitor| visitor.visit_unsupported_item(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }

    fn visit_unsupported_impl_item(
        &mut self,
        korok: &mut UnsupportedImplItemKorok,
    ) -> CodamaResult<()> {
        self.visitors
            .iter_mut()
            .map(|visitor| visitor.visit_unsupported_impl_item(korok))
            .collect_and_combine_errors()?;
        Ok(())
    }
}
