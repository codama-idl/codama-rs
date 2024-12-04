use codama_errors::{CodamaError, CodamaResult};
use codama_korok_plugins::{resolve_plugins, KorokPlugin};
use codama_koroks::RootKorok;
use codama_nodes::{Node, NodeUnionTrait, RootNode};
use codama_stores::RootStore;

pub struct Codama {
    store: RootStore,
    plugins: Vec<Box<dyn KorokPlugin>>,
}

impl Codama {
    pub fn new(store: RootStore) -> Self {
        Self {
            store,
            plugins: Vec::new(),
        }
    }

    pub fn add_plugin<T: KorokPlugin + 'static>(mut self, plugin: T) -> Self {
        self.plugins.push(Box::new(plugin));
        self
    }

    pub fn get_korok(&self) -> CodamaResult<RootKorok> {
        RootKorok::parse(&self.store)
    }

    pub fn get_visited_korok(&self) -> CodamaResult<RootKorok> {
        let mut korok = self.get_korok()?;
        let run_plugins = resolve_plugins(&self.plugins);
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
}
