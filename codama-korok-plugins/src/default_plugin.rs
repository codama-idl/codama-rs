use crate::KorokPlugin;
use codama_errors::CodamaResult;
use codama_korok_visitors::{
    ApplyCodamaTypeAttributesVisitor, CombineModulesVisitor, ComposeVisitor, FilterItemsVisitor,
    KorokVisitable, SetAccountsVisitor, SetBorshTypesVisitor, SetDefinedTypesVisitor,
    SetErrorsVisitor, SetInstructionsVisitor, SetLinkTypesVisitor, SetProgramMetadataVisitor,
};
use codama_koroks::KorokTrait;

pub struct DefaultPlugin;
impl KorokPlugin for DefaultPlugin {
    fn run(
        &self,
        visitable: &mut dyn KorokVisitable,
        next: &dyn Fn(&mut dyn KorokVisitable) -> CodamaResult<()>,
    ) -> CodamaResult<()> {
        next(visitable)?;
        visitable.accept(&mut get_default_visitor())?;
        Ok(())
    }
}

pub fn get_default_visitor<'a>() -> ComposeVisitor<'a> {
    ComposeVisitor::new()
        .with(FilterItemsVisitor::new(
            |item| item.attributes().unwrap().has_any_codama_derive(),
            ComposeVisitor::new()
                .with(SetBorshTypesVisitor::new())
                .with(SetLinkTypesVisitor::new()),
        ))
        .with(SetProgramMetadataVisitor::new())
        .with(ApplyCodamaTypeAttributesVisitor::new())
        .with(SetDefinedTypesVisitor::new())
        .with(SetAccountsVisitor::new())
        .with(SetInstructionsVisitor::new())
        .with(SetErrorsVisitor::new())
        .with(CombineModulesVisitor::new())
}
