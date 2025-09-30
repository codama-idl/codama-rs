use crate::KorokPlugin;
use codama_errors::CodamaResult;
use codama_korok_visitors::{
    ApplyTypeModifiersVisitor, ApplyTypeOverridesVisitor, CombineModulesVisitor, KorokVisitable,
    SetAccountsVisitor, SetBorshTypesVisitor, SetDefinedTypesVisitor, SetErrorsVisitor,
    SetInstructionsVisitor, SetLinkTypesVisitor, SetProgramMetadataVisitor,
};

pub struct DefaultPlugin;
impl KorokPlugin for DefaultPlugin {
    fn on_fields_set(&self, visitable: &mut dyn KorokVisitable) -> CodamaResult<()> {
        visitable.accept(&mut SetBorshTypesVisitor::new())?;
        visitable.accept(&mut SetLinkTypesVisitor::new())?;
        visitable.accept(&mut ApplyTypeOverridesVisitor::new())?;
        visitable.accept(&mut ApplyTypeModifiersVisitor::new())?;
        Ok(())
    }

    fn on_program_items_set(&self, visitable: &mut dyn KorokVisitable) -> CodamaResult<()> {
        visitable.accept(&mut SetDefinedTypesVisitor::new())?;
        visitable.accept(&mut SetAccountsVisitor::new())?;
        visitable.accept(&mut SetInstructionsVisitor::new())?;
        visitable.accept(&mut SetErrorsVisitor::new())?;
        Ok(())
    }

    fn on_root_node_set(&self, visitable: &mut dyn KorokVisitable) -> CodamaResult<()> {
        visitable.accept(&mut SetProgramMetadataVisitor::new())?;
        visitable.accept(&mut CombineModulesVisitor::new())?;
        Ok(())
    }
}
