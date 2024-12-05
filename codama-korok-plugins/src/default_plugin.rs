use crate::KorokPlugin;
use codama_korok_visitors::{
    CombineModulesVisitor, CombineTypesVisitor, ComposeVisitor, KorokVisitable,
    SetBorshTypesVisitor, SetLinkTypesVisitor, SetProgramMetadataVisitor,
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
        .add(SetBorshTypesVisitor::new())
        .add(SetLinkTypesVisitor::new())
        .add(SetProgramMetadataVisitor::new())
        .add(CombineTypesVisitor::new())
        .add(CombineModulesVisitor::new())
}
