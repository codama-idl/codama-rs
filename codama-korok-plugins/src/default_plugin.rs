use crate::KorokPlugin;
use codama_korok_visitors::{
    CombineModulesVisitor, CombineTypesVisitor, ComposeVisitor, DefineBorshTypesVisitor,
    DefineLinkTypesVisitor, KorokVisitable,
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
        .add(DefineBorshTypesVisitor::new())
        .add(DefineLinkTypesVisitor::new())
        .add(CombineTypesVisitor::new())
        .add(CombineModulesVisitor::new())
}
