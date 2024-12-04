use crate::KorokPlugin;
use codama_korok_visitors::{
    BorshVisitor, CombineModulesVisitor, CombineTypesVisitor, ComposeVisitor, KorokVisitable,
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
        .add(BorshVisitor::new())
        .add(CombineTypesVisitor::new())
        .add(CombineModulesVisitor::new())
}
