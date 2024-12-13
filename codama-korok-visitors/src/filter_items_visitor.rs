use crate::KorokVisitor;
use codama_koroks::ItemKorok;

pub struct FilterItemsVisitor<'a> {
    pub filter: fn(item: &ItemKorok) -> bool,
    pub visitor: Box<dyn KorokVisitor + 'a>,
}

impl<'a> FilterItemsVisitor<'a> {
    pub fn new<T: KorokVisitor + 'a>(filter: fn(item: &ItemKorok) -> bool, visitor: T) -> Self {
        Self {
            filter,
            visitor: Box::new(visitor),
        }
    }
}

impl KorokVisitor for FilterItemsVisitor<'_> {
    fn visit_item(&mut self, korok: &mut codama_koroks::ItemKorok) {
        if (self.filter)(korok) {
            self.visitor.visit_item(korok);
        } else {
            self.visit_children(korok);
        }
    }
}
