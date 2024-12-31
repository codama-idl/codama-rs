use crate::KorokPlugin;
use codama_errors::CodamaResult;
use codama_korok_visitors::{
    ApplyCodamaTypeAttributesVisitor, CombineModulesVisitor, CombineTypesVisitor, ComposeVisitor,
    FilterItemsVisitor, KorokVisitable, SetAccountsVisitor, SetBorshTypesVisitor,
    SetLinkTypesVisitor, SetProgramMetadataVisitor,
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
        .add(FilterItemsVisitor::new(
            |item| item.attributes().unwrap().has_any_codama_derive(),
            ComposeVisitor::new()
                .add(SetBorshTypesVisitor::new())
                .add(SetLinkTypesVisitor::new()),
        ))
        .add(SetProgramMetadataVisitor::new())
        .add(ApplyCodamaTypeAttributesVisitor::new())
        .add(FilterItemsVisitor::new(
            |item| item.attributes().unwrap().has_any_codama_derive(),
            ComposeVisitor::new()
                .add(CombineTypesVisitor::new())
                .add(SetAccountsVisitor::new()),
        ))
        .add(CombineModulesVisitor::new())
}
