use cargo_toml::{Inheritable, Manifest, Package, Value};
use codama_koroks::{CrateKorok, UnsupportedItemKorok};
use codama_nodes::{Node, ProgramNode};
use codama_syn_helpers::extensions::*;

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
            if let Some(p) = get_package(&korok.store.manifest) {
                program.name = p.name.clone().into()
            }
        }

        // Update the version using the Cargo.toml package version.
        // E.g. `version = "0.1.0"`
        if program.version.is_empty() {
            if let Some(Package {
                version: Inheritable::Set(version),
                ..
            }) = get_package(&korok.store.manifest)
            {
                program.version = version.clone()
            }
        }

        // Update the program ID using the Cargo.toml metadata.
        // E.g.
        // [package.metadata.solana]
        // program-id = "AddressLookupTab1e1111111111111111111111111"
        if program.public_key.is_empty() {
            if let Some(public_key) = get_metadata_solana_program_id(&korok.store.manifest) {
                program.public_key = public_key.into()
            }
        }

        // Update the program ID using the `solana_program::declare_id!` macro.
        // E.g. `solana_program::declare_id!("AddressLookupTab1e1111111111111111111111111");`
        if program.public_key.is_empty() {
            if let Some(public_key) = &self.identified_public_key {
                program.public_key = public_key.into()
            }
        }
    }

    fn visit_unsupported_item(&mut self, korok: &mut UnsupportedItemKorok) {
        let syn::Item::Macro(syn::ItemMacro { mac, .. }) = korok.ast else {
            return;
        };

        if let ("" | "solana_program", "declare_id") =
            (mac.path.prefix().as_str(), mac.path.last_str().as_str())
        {
            self.identified_public_key = Some(mac.tokens.to_string().replace("\"", ""));
        };
    }
}

fn get_package(manifest: &Option<Manifest>) -> Option<&Package> {
    match &manifest {
        Some(Manifest { package, .. }) => package.as_ref(),
        _ => None,
    }
}

fn get_metadata(manifest: &Option<Manifest>) -> Option<&Value> {
    match get_package(manifest) {
        Some(Package { metadata, .. }) => metadata.as_ref(),
        _ => None,
    }
}

fn get_metadata_solana_program_id(manifest: &Option<Manifest>) -> Option<&str> {
    match get_metadata(manifest) {
        Some(metadata) => metadata.get("solana")?.get("program-id")?.as_str(),
        _ => None,
    }
}
