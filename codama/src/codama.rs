use codama_errors::{CodamaError, CodamaResult};
use codama_koroks::RootKorok;
use codama_nodes::{HasKind, Node, NodeTrait, RootNode};
use codama_plugin_core::{resolve_plugins, DefaultPlugin, KorokPlugin};
use codama_stores::RootStore;
use codama_validators::{get_validation_items_visitor, throw_validator_items_visitor, LogLevel};
use codama_visitors_core::visit;
use std::path::Path;

pub struct Codama {
    store: RootStore,
    plugins: Vec<Box<dyn KorokPlugin>>,
    with_default_plugin: bool,
}

impl Codama {
    pub fn new(store: RootStore) -> Self {
        Self {
            store,
            plugins: Vec::new(),
            with_default_plugin: true,
        }
        .add_plugin(DefaultPlugin)
    }

    pub fn load<P: AsRef<Path>>(path: P) -> CodamaResult<Self> {
        Ok(Self::new(RootStore::load(path)?))
    }

    pub fn load_all(paths: &[&Path]) -> CodamaResult<Self> {
        Ok(Self::new(RootStore::load_all(paths)?))
    }

    pub fn hydrate(tt: proc_macro2::TokenStream) -> CodamaResult<Self> {
        Ok(Self::new(RootStore::hydrate(tt)?))
    }

    pub fn without_default_plugin(mut self) -> Self {
        self.with_default_plugin = false;
        self
    }

    pub fn add_plugin<T: KorokPlugin + 'static>(mut self, plugin: T) -> Self {
        self.plugins.push(Box::new(plugin));
        self
    }

    pub fn get_korok(&'_ self) -> CodamaResult<RootKorok<'_>> {
        RootKorok::parse(&self.store)
    }

    pub fn get_plugins(&self) -> &[Box<dyn KorokPlugin>] {
        match self.with_default_plugin {
            true => &self.plugins,
            false => &self.plugins[1..],
        }
    }

    pub fn get_visited_korok(&'_ self) -> CodamaResult<RootKorok<'_>> {
        let mut korok = self.get_korok()?;
        let run_plugins = resolve_plugins(self.get_plugins());
        run_plugins(&mut korok)?;
        Ok(korok)
    }

    /// The node of the visited crates, once every plugin has run.
    ///
    /// The node is validated with `get_validation_items_visitor`, and any error-level item fails
    /// with `CodamaError::FailedToValidateNode`, which lists every item.
    pub fn get_node(&self) -> CodamaResult<Node> {
        let korok = self.get_visited_korok()?;
        let node = korok.node.clone().ok_or(CodamaError::NodeNotFound)?;
        {
            // The validator borrows the node, so it is dropped before the node is returned.
            let validator =
                throw_validator_items_visitor(get_validation_items_visitor(), LogLevel::Error);
            visit(&validator, &node)?;
        }
        Ok(node)
    }

    pub fn get_idl(&self) -> CodamaResult<RootNode> {
        let node = self.get_node()?;
        match node {
            Node::Root(root) => Ok(root),
            _ => Err(CodamaError::UnexpectedNode {
                expected: "RootNode".to_string(),
                actual: node.kind().to_string(),
            }),
        }
    }

    pub fn get_json_idl(&self) -> CodamaResult<String> {
        self.get_idl()?.to_json()
    }
}
