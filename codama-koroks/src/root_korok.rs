use crate::{CrateKorok, KorokTrait};
use codama_errors::{CodamaResult, IteratorCombineErrors};
use codama_nodes::Node;
use codama_stores::RootStore;

#[derive(Debug, PartialEq)]
pub struct RootKorok<'a> {
    pub crates: Vec<CrateKorok<'a>>,
    pub node: Option<Node>,
    pub store: &'a RootStore,
}

impl<'a> RootKorok<'a> {
    pub fn parse(root_store: &'a RootStore) -> CodamaResult<Self> {
        Ok(Self {
            crates: root_store
                .crates
                .iter()
                .map(CrateKorok::parse)
                .collect_and_combine_errors()?,
            node: None,
            store: root_store,
        })
    }
}

impl KorokTrait for RootKorok<'_> {
    fn node(&self) -> &Option<Node> {
        &self.node
    }

    fn set_node(&mut self, node: Option<Node>) {
        self.node = node;
    }
}
