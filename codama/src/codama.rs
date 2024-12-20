use codama_errors::{CodamaError, CodamaResult};
use codama_korok_plugins::{resolve_plugins, DefaultPlugin, KorokPlugin};
use codama_koroks::RootKorok;
use codama_nodes::{HasKind, Node, NodeTrait, RootNode};
use codama_stores::RootStore;
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

    pub fn get_korok(&self) -> CodamaResult<RootKorok> {
        RootKorok::parse(&self.store)
    }

    pub fn get_plugins(&self) -> &[Box<dyn KorokPlugin>] {
        match self.with_default_plugin {
            true => &self.plugins,
            false => &self.plugins[1..],
        }
    }

    pub fn get_visited_korok(&self) -> CodamaResult<RootKorok> {
        let mut korok = self.get_korok()?;
        let run_plugins = resolve_plugins(self.get_plugins());
        run_plugins(&mut korok);
        Ok(korok)
    }

    pub fn get_node(&self) -> CodamaResult<Node> {
        let korok = self.get_visited_korok()?;
        korok.node.clone().ok_or(CodamaError::NodeNotFound)
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
