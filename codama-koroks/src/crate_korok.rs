use crate::ItemKorok;
use codama_errors::CodamaResult;
use codama_nodes::Node;
use codama_stores::CrateStore;

#[derive(Debug)]
pub struct CrateKorok<'a> {
    pub items: Vec<ItemKorok<'a>>,
    pub node: Option<Node>,
    pub store: &'a CrateStore,
}

impl<'a> CrateKorok<'a> {
    pub fn parse(crate_store: &'a CrateStore) -> CodamaResult<Self> {
        Ok(Self {
            items: ItemKorok::parse_all(
                &crate_store.file.items,
                &crate_store.file_modules,
                &mut 0,
            )?,
            node: None,
            store: crate_store,
        })
    }
}
