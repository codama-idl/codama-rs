use codama_korok_visitors::{KorokVisitable, SetProgramMetadataVisitor};
use codama_koroks::CrateKorok;
use codama_nodes::{Node, ProgramNode, RootNode, StringValueNode};
use codama_stores::CrateStore;
use quote::quote;

#[test]
fn it_gets_program_metadata_from_the_manifest() {
    let mut store = CrateStore::hydrate(quote! {}).unwrap();
    let manifest = cargo_toml::Manifest::from_path(get_path("full_metadata.toml")).unwrap();
    store.manifest = Some(manifest);

    let mut korok = CrateKorok::parse(&store).unwrap();
    korok.accept(&mut SetProgramMetadataVisitor::new());

    let Some(Node::Program(program)) = korok.node else {
        panic!("Expected program node");
    };

    assert_eq!(program.name, "myCrateName".into());
    assert_eq!(program.version, "1.2.3");
    assert_eq!(
        program.public_key,
        "MyProgramAddress1111111111111111111111111"
    );
}

#[test]
fn it_gets_program_ids_from_the_declare_id_macro() {
    let store = CrateStore::hydrate(quote! {
        solana_program::declare_id!("MyProgramAddress1111111111111111111111111");
    })
    .unwrap();
    let mut korok = CrateKorok::parse(&store).unwrap();
    korok.accept(&mut SetProgramMetadataVisitor::new());

    let Some(Node::Program(program)) = korok.node else {
        panic!("Expected program node");
    };
    assert_eq!(
        program.public_key,
        "MyProgramAddress1111111111111111111111111"
    );
}

#[test]
fn it_prioritises_the_program_id_from_the_manifest() {
    let mut store = CrateStore::hydrate(quote! {
        solana_program::declare_id!("MyMacroProgramAddress1111111111111111111111111");
    })
    .unwrap();
    let manifest = cargo_toml::Manifest::from_path(get_path("full_metadata.toml")).unwrap();
    store.manifest = Some(manifest);

    let mut korok = CrateKorok::parse(&store).unwrap();
    korok.accept(&mut SetProgramMetadataVisitor::new());

    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                name: "myCrateName".into(),
                version: "1.2.3".into(),
                public_key: "MyProgramAddress1111111111111111111111111".into(),
                ..Default::default()
            }
            .into()
        )
    );
}

#[test]
fn it_updates_existing_program_nodes() {
    let store = CrateStore::hydrate(quote! {
        solana_program::declare_id!("MyProgramAddress1111111111111111111111111");
    })
    .unwrap();

    let mut korok = CrateKorok::parse(&store).unwrap();
    korok.node = Some(ProgramNode::default().into());
    korok.accept(&mut SetProgramMetadataVisitor::new());

    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                public_key: "MyProgramAddress1111111111111111111111111".into(),
                ..Default::default()
            }
            .into()
        )
    );
}

#[test]
fn it_updates_the_primary_program_of_existing_root_nodes() {
    let store = CrateStore::hydrate(quote! {
        solana_program::declare_id!("MyProgramAddress1111111111111111111111111");
    })
    .unwrap();

    let mut korok = CrateKorok::parse(&store).unwrap();
    korok.node = Some(RootNode::default().into());
    korok.accept(&mut SetProgramMetadataVisitor::new());

    assert_eq!(
        korok.node,
        Some(
            RootNode::new(ProgramNode {
                public_key: "MyProgramAddress1111111111111111111111111".into(),
                ..Default::default()
            })
            .into()
        )
    );
}

#[test]
fn it_does_not_override_existing_values() {
    let mut store = CrateStore::hydrate(quote! {
        solana_program::declare_id!("MyMacroProgramAddress1111111111111111111111111");
    })
    .unwrap();
    let manifest = cargo_toml::Manifest::from_path(get_path("full_metadata.toml")).unwrap();
    store.manifest = Some(manifest);

    let mut korok = CrateKorok::parse(&store).unwrap();
    let existing_program = ProgramNode {
        name: "myExistingName".into(),
        version: "9.9.9".into(),
        public_key: "MyExistingProgramAddress1111111111111111111111111".into(),
        ..Default::default()
    };
    korok.node = Some(existing_program.clone().into());

    korok.accept(&mut SetProgramMetadataVisitor::new());
    assert_eq!(korok.node, Some(existing_program.into()));
}

#[test]
fn it_does_nothing_to_existing_nodes_that_are_not_roots_or_programs() {
    let store = CrateStore::hydrate(quote! {
        solana_program::declare_id!("MyProgramAddress1111111111111111111111111");
    })
    .unwrap();

    let mut korok = CrateKorok::parse(&store).unwrap();
    korok.node = Some(StringValueNode::new("hello").into());

    korok.accept(&mut SetProgramMetadataVisitor::new());
    assert_eq!(korok.node, Some(StringValueNode::new("hello").into()));
}

pub fn get_path(relative_path: &str) -> std::path::PathBuf {
    let project_dir = env!("CARGO_MANIFEST_DIR");
    std::path::Path::new(project_dir)
        .join("tests")
        .join("set_program_metadata_visitor")
        .join("fixtures")
        .join(relative_path)
}
