use crate::KorokPlugin;
use codama_korok_visitors::{
    ApplyCodamaAttributesVisitor, CombineModulesVisitor, CombineTypesVisitor, ComposeVisitor,
    FilterItemsVisitor, KorokVisitable, SetBorshTypesVisitor, SetLinkTypesVisitor,
    SetProgramMetadataVisitor,
};

pub struct DefaultPlugin;
impl KorokPlugin for DefaultPlugin {
    fn run(&self, visitable: &mut dyn KorokVisitable, next: &dyn Fn(&mut dyn KorokVisitable)) {
        next(visitable);
        visitable.accept(&mut get_default_visitor());
    }
}

pub fn get_default_visitor<'a>() -> ComposeVisitor<'a> {
    ComposeVisitor::new()
        .add(FilterItemsVisitor::new(
            |item| item.attributes().has_any_codama_derive(),
            ComposeVisitor::new()
                .add(SetBorshTypesVisitor::new())
                .add(SetLinkTypesVisitor::new()),
        ))
        .add(SetProgramMetadataVisitor::new())
        .add(ApplyCodamaAttributesVisitor::new())
        .add(FilterItemsVisitor::new(
            |item| item.attributes().has_any_codama_derive(),
            CombineTypesVisitor::new(),
        ))
        .add(CombineModulesVisitor::new())
}
