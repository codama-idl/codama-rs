use codama_attributes::Attributes;
use codama_korok_visitors::{CombineModulesVisitor, KorokVisitable};
use codama_koroks::{ItemKorok, ModuleKorok, UnsupportedItemKorok};
use codama_nodes::{DefinedTypeNode, Node, PublicKeyTypeNode};
use codama_stores::CrateStore;
use quote::{format_ident, quote};
use std::default;

pub struct CombineModulesInput {
    pub name: String,
    pub initial_node: Option<Node>,
    pub nodes: Vec<Option<Node>>,
}

impl default::Default for CombineModulesInput {
    fn default() -> Self {
        Self {
            name: "my_module".to_string(),
            initial_node: None,
            nodes: vec![],
        }
    }
}

impl CombineModulesInput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_initial_node<T: Into<Node>>(mut self, node: T) -> Self {
        self.initial_node = Some(node.into());
        self
    }

    pub fn add_node<T: Into<Node>>(mut self, node: T) -> Self {
        self.nodes.push(Some(node.into()));
        self
    }
}

pub fn combine_modules<'a>(input: CombineModulesInput) -> Option<Node> {
    let name = format_ident!("{}", input.name);
    let crate_store = CrateStore::hydrate(quote! {
        mod #name {
            type Foo = ();
        }
    })
    .unwrap();

    let [syn::Item::Mod(item_mod)] = &crate_store.file.items.as_slice() else {
        panic!("Expected to find a module node");
    };
    let Some((_, inner_items)) = &item_mod.content else {
        panic!("Expected to find a module node with content");
    };
    let [inner_item] = &inner_items.as_slice() else {
        panic!("Expected to find a single item in the module");
    };

    let items = input
        .nodes
        .iter()
        .map(|node| {
            ItemKorok::Unsupported(UnsupportedItemKorok {
                ast: inner_item,
                attributes: Attributes(Vec::new()),
                node: node.clone(),
            })
        })
        .collect::<Vec<_>>();

    let mut module_korok = ModuleKorok {
        ast: item_mod,
        attributes: Attributes(Vec::new()),
        items,
        node: None,
    };

    module_korok.node = input.initial_node;
    module_korok
        .accept(&mut CombineModulesVisitor::new())
        .unwrap();
    module_korok.node
}

pub fn get_type(name: &str) -> DefinedTypeNode {
    DefinedTypeNode::new(name, PublicKeyTypeNode::new())
}
