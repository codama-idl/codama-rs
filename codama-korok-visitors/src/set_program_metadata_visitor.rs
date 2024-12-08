use cargo_toml::{Inheritable, Manifest, Package, Value};
use codama_koroks::{CrateKorok, UnsupportedItemKorok};
use codama_nodes::{Node, ProgramNode};
use codama_syn_helpers::syn_traits::Path;

use crate::KorokVisitor;

/// Fill program metadata using the Cargo.toml manifest and the `solana_program::declare_id!` macro.
#[derive(Default)]
pub struct SetProgramMetadataVisitor {
    identified_public_key: Option<String>,
}

impl SetProgramMetadataVisitor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl KorokVisitor for SetProgramMetadataVisitor {
    fn visit_crate(&mut self, korok: &mut CrateKorok) {
        self.visit_children(korok);

        // Get a mutable reference to the program to update its metadata.
        let program = match &mut korok.node {
            // Use the primary program of the root node if set.
            Some(Node::Root(root)) => &mut root.program,
            // Use the existing program node if set.
            Some(Node::Program(program)) => program,
            // If no node is set, create a new default program node.
            None => {
                korok.node = Some(ProgramNode::default().into());
                if let Some(Node::Program(program)) = &mut korok.node {
                    program
                } else {
                    unreachable!()
                }
            }
            // Don't update the node if it is set to anything else.
            _ => return,
        };

        // Update the program name using the Cargo.toml package name.
        // E.g. `name = "my-program"`
        if program.name.is_empty() {
            match get_package(&korok.store.manifest) {
                Some(p) => program.name = p.name.clone().into(),
                _ => (),
            }
        }

        // Update the version using the Cargo.toml package version.
        // E.g. `version = "0.1.0"`
        if program.version.is_empty() {
            match get_package(&korok.store.manifest) {
                Some(Package {
                    version: Inheritable::Set(version),
                    ..
                }) => program.version = version.clone().into(),
                _ => (),
            }
        }

        // Update the program ID using the Cargo.toml metadata.
        // E.g.
        // [package.metadata.solana]
        // program-id = "AddressLookupTab1e1111111111111111111111111"
        if program.public_key.is_empty() {
            match get_metadata_solana_program_id(&korok.store.manifest) {
                Some(public_key) => program.public_key = public_key.into(),
                _ => (),
            }
        }

        // Update the program ID using the `solana_program::declare_id!` macro.
        // E.g. `solana_program::declare_id!("AddressLookupTab1e1111111111111111111111111");`
        if program.public_key.is_empty() {
            match &self.identified_public_key {
                Some(public_key) => program.public_key = public_key.into(),
                _ => (),
            }
        }
    }

    fn visit_unsupported_item(&mut self, korok: &mut UnsupportedItemKorok) {
        let syn::Item::Macro(syn::ItemMacro { mac, .. }) = korok.ast else {
            return;
        };

        match (mac.path.prefix().as_str(), mac.path.last_str().as_str()) {
            ("" | "solana_program", "declare_id") => {
                self.identified_public_key = Some(mac.tokens.to_string().replace("\"", ""));
            }
            _ => (),
        };
    }
}

fn get_package<'a>(manifest: &'a Option<Manifest>) -> Option<&'a Package> {
    match &manifest {
        Some(Manifest { package, .. }) => package.as_ref(),
        _ => None,
    }
}

fn get_metadata<'a>(manifest: &'a Option<Manifest>) -> Option<&'a Value> {
    match get_package(manifest) {
        Some(Package { metadata, .. }) => metadata.as_ref(),
        _ => None,
    }
}

fn get_metadata_solana_program_id<'a>(manifest: &'a Option<Manifest>) -> Option<&str> {
    match get_metadata(manifest) {
        Some(metadata) => metadata.get("solana")?.get("program-id")?.as_str(),
        _ => None,
    }
}
