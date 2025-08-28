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
    pub fn parse(store: &'a CrateStore) -> CodamaResult<Self> {
        let (attributes, items) = combine_errors!(
            Attributes::parse(&store.file.attrs, (&store.file).into()).map_err(CodamaError::from),
            ItemKorok::parse_all(&store.file.items, &store.file_modules, &mut 0,),
        )?;
        Ok(Self {
            attributes,
            items,
            node: None,
            store,
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

    fn attributes(&self) -> Option<&Attributes<'_>> {
        Some(&self.attributes)
    }
}
