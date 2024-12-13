use crate::{ItemKorok, KorokTrait};
use codama_attributes::Attributes;
use codama_errors::{combine_errors, CodamaError, CodamaResult};
use codama_nodes::Node;
use codama_stores::CrateStore;

#[derive(Debug, PartialEq)]
pub struct CrateKorok<'a> {
    pub attributes: Attributes<'a>,
    pub items: Vec<ItemKorok<'a>>,
    pub node: Option<Node>,
    pub store: &'a CrateStore,
}

impl<'a> CrateKorok<'a> {
    pub fn parse(crate_store: &'a CrateStore) -> CodamaResult<Self> {
        let (attributes, items) = combine_errors!(
            Attributes::parse(&crate_store.file.attrs).map_err(CodamaError::from),
            ItemKorok::parse_all(&crate_store.file.items, &crate_store.file_modules, &mut 0,),
        )?;
        Ok(Self {
            attributes,
            items,
            node: None,
            store: crate_store,
        })
    }
}

impl KorokTrait for CrateKorok<'_> {
    fn node(&self) -> &Option<Node> {
        &self.node
    }

    fn set_node(&mut self, node: Option<Node>) {
        self.node = node;
    }
}
