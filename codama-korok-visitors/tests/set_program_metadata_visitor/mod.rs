use codama_errors::CodamaResult;
use codama_korok_visitors::{KorokVisitable, SetProgramMetadataVisitor};
use codama_koroks::CrateKorok;
use codama_nodes::{Node, ProgramNode, RootNode, StringValueNode};
use codama_stores::CrateStore;
use quote::quote;

#[test]
fn it_gets_program_metadata_from_the_manifest() -> CodamaResult<()> {
    let mut store = CrateStore::hydrate(quote! {})?;
    let manifest = cargo_toml::Manifest::from_path(get_path("full_metadata.toml"))?;
    store.manifest = Some(manifest);

    let mut korok = CrateKorok::parse(&store)?;
    korok.accept(&mut SetProgramMetadataVisitor::new())?;

    let Some(Node::Program(program)) = korok.node else {
        panic!("Expected program node");
    };

    assert_eq!(program.name, "myCrateName".into());
    assert_eq!(program.version, "1.2.3");
    assert_eq!(
        program.public_key,
        "MyProgramAddress1111111111111111111111111"
    );
    Ok(())
}

#[test]
fn it_gets_program_ids_from_the_declare_id_macro() -> CodamaResult<()> {
    let store = CrateStore::hydrate(quote! {
        solana_program::declare_id!("MyProgramAddress1111111111111111111111111");
    })?;
    let mut korok = CrateKorok::parse(&store)?;
    korok.accept(&mut SetProgramMetadataVisitor::new())?;

    let Some(Node::Program(program)) = korok.node else {
        panic!("Expected program node");
    };
    assert_eq!(
        program.public_key,
        "MyProgramAddress1111111111111111111111111"
    );
    Ok(())
}

#[test]
fn it_prioritises_the_program_id_from_the_manifest() -> CodamaResult<()> {
    let mut store = CrateStore::hydrate(quote! {
        solana_program::declare_id!("MyMacroProgramAddress1111111111111111111111111");
    })?;
    let manifest = cargo_toml::Manifest::from_path(get_path("full_metadata.toml"))?;
    store.manifest = Some(manifest);

    let mut korok = CrateKorok::parse(&store)?;
    korok.accept(&mut SetProgramMetadataVisitor::new())?;

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
    Ok(())
}

#[test]
fn it_updates_existing_program_nodes() -> CodamaResult<()> {
    let store = CrateStore::hydrate(quote! {
        solana_program::declare_id!("MyProgramAddress1111111111111111111111111");
    })?;

    let mut korok = CrateKorok::parse(&store)?;
    korok.node = Some(ProgramNode::default().into());
    korok.accept(&mut SetProgramMetadataVisitor::new())?;

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
    Ok(())
}

#[test]
fn it_updates_the_primary_program_of_existing_root_nodes() -> CodamaResult<()> {
    let store = CrateStore::hydrate(quote! {
        solana_program::declare_id!("MyProgramAddress1111111111111111111111111");
    })?;

    let mut korok = CrateKorok::parse(&store)?;
    korok.node = Some(RootNode::default().into());
    korok.accept(&mut SetProgramMetadataVisitor::new())?;

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
    Ok(())
}

#[test]
fn it_does_not_override_existing_values() -> CodamaResult<()> {
    let mut store = CrateStore::hydrate(quote! {
        solana_program::declare_id!("MyMacroProgramAddress1111111111111111111111111");
    })?;
    let manifest = cargo_toml::Manifest::from_path(get_path("full_metadata.toml"))?;
    store.manifest = Some(manifest);

    let mut korok = CrateKorok::parse(&store)?;
    let existing_program = ProgramNode {
        name: "myExistingName".into(),
        version: "9.9.9".into(),
        public_key: "MyExistingProgramAddress1111111111111111111111111".into(),
        ..Default::default()
    };
    korok.node = Some(existing_program.clone().into());

    korok.accept(&mut SetProgramMetadataVisitor::new())?;
    assert_eq!(korok.node, Some(existing_program.into()));
    Ok(())
}

#[test]
fn it_does_nothing_to_existing_nodes_that_are_not_roots_or_programs() -> CodamaResult<()> {
    let store = CrateStore::hydrate(quote! {
        solana_program::declare_id!("MyProgramAddress1111111111111111111111111");
    })?;

    let mut korok = CrateKorok::parse(&store)?;
    korok.node = Some(StringValueNode::new("hello").into());

    korok.accept(&mut SetProgramMetadataVisitor::new())?;
    assert_eq!(korok.node, Some(StringValueNode::new("hello").into()));
    Ok(())
}

#[test]
fn it_overrides_the_program_name_from_a_codama_program_macro() -> CodamaResult<()> {
    // The macro omits the address, which therefore still resolves from `declare_id!`.
    let store = CrateStore::hydrate(quote! {
        solana_program::declare_id!("MyProgramAddress1111111111111111111111111");
        codama_program!(name = "myOverriddenProgramName");
    })?;

    let mut korok = CrateKorok::parse(&store)?;
    korok.accept(&mut SetProgramMetadataVisitor::new())?;

    let Some(Node::Program(program)) = korok.node else {
        panic!("Expected program node");
    };
    assert_eq!(program.name, "myOverriddenProgramName".into());
    assert_eq!(
        program.public_key,
        "MyProgramAddress1111111111111111111111111"
    );
    Ok(())
}

#[test]
fn it_overrides_the_program_name_over_the_manifest_default() -> CodamaResult<()> {
    let mut store = CrateStore::hydrate(quote! {
        codama_program!(name = "myOverriddenProgramName");
    })?;
    let manifest = cargo_toml::Manifest::from_path(get_path("full_metadata.toml"))?;
    store.manifest = Some(manifest);

    let mut korok = CrateKorok::parse(&store)?;
    korok.accept(&mut SetProgramMetadataVisitor::new())?;

    let Some(Node::Program(program)) = korok.node else {
        panic!("Expected program node");
    };
    // The macro wins the name; the address and version still come from the
    // manifest since the macro omitted the address.
    assert_eq!(program.name, "myOverriddenProgramName".into());
    assert_eq!(program.version, "1.2.3");
    assert_eq!(
        program.public_key,
        "MyProgramAddress1111111111111111111111111"
    );
    Ok(())
}

#[test]
fn it_overrides_the_program_address_from_a_codama_program_macro() -> CodamaResult<()> {
    let store = CrateStore::hydrate(quote! {
        solana_program::declare_id!("MyMacroProgramAddress1111111111111111111111111");
        codama_program!(name = "myOverriddenProgramName", address = "MyOverrideAddress11111111111111111111111111");
    })?;

    let mut korok = CrateKorok::parse(&store)?;
    korok.accept(&mut SetProgramMetadataVisitor::new())?;

    let Some(Node::Program(program)) = korok.node else {
        panic!("Expected program node");
    };
    // Both the name and address come from the macro, overriding `declare_id!`.
    assert_eq!(program.name, "myOverriddenProgramName".into());
    assert_eq!(
        program.public_key,
        "MyOverrideAddress11111111111111111111111111"
    );
    Ok(())
}

#[test]
fn it_overrides_only_the_program_address_from_a_codama_program_macro() -> CodamaResult<()> {
    // An address-only macro leaves the name to the manifest default.
    let mut store = CrateStore::hydrate(quote! {
        codama_program!(address = "MyOverrideAddress11111111111111111111111111");
    })?;
    let manifest = cargo_toml::Manifest::from_path(get_path("full_metadata.toml"))?;
    store.manifest = Some(manifest);

    let mut korok = CrateKorok::parse(&store)?;
    korok.accept(&mut SetProgramMetadataVisitor::new())?;

    let Some(Node::Program(program)) = korok.node else {
        panic!("Expected program node");
    };
    assert_eq!(program.name, "myCrateName".into());
    assert_eq!(
        program.public_key,
        "MyOverrideAddress11111111111111111111111111"
    );
    Ok(())
}

#[test]
fn it_fails_on_repeated_codama_program_macros() -> CodamaResult<()> {
    let store = CrateStore::hydrate(quote! {
        codama_program!(name = "firstName");
        codama_program!(name = "secondName");
    })?;

    let mut korok = CrateKorok::parse(&store)?;
    let error = korok
        .accept(&mut SetProgramMetadataVisitor::new())
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "`codama_program!` can only be used once per crate"
    );
    Ok(())
}

#[test]
fn it_ignores_codama_program_macros_from_other_crates() -> CodamaResult<()> {
    // Only bare, `codama::` and `codama_macros::` prefixes are recognised, so a
    // foreign `codama_program!` is ignored — even one whose arguments would not
    // parse as ours.
    let mut store = CrateStore::hydrate(quote! {
        other_crate::codama_program!(this = "would not parse");
    })?;
    let manifest = cargo_toml::Manifest::from_path(get_path("full_metadata.toml"))?;
    store.manifest = Some(manifest);

    let mut korok = CrateKorok::parse(&store)?;
    korok.accept(&mut SetProgramMetadataVisitor::new())?;

    let Some(Node::Program(program)) = korok.node else {
        panic!("Expected program node");
    };
    assert_eq!(program.name, "myCrateName".into());
    Ok(())
}

#[test]
fn it_gets_program_ids_from_the_solana_address_declare_id_macro() -> CodamaResult<()> {
    let store = CrateStore::hydrate(quote! {
        solana_address::declare_id!("MyProgramAddress1111111111111111111111111");
    })?;
    let mut korok = CrateKorok::parse(&store)?;
    korok.accept(&mut SetProgramMetadataVisitor::new())?;

    let Some(Node::Program(program)) = korok.node else {
        panic!("Expected program node");
    };
    assert_eq!(
        program.public_key,
        "MyProgramAddress1111111111111111111111111"
    );
    Ok(())
}

pub fn get_path(relative_path: &str) -> std::path::PathBuf {
    let project_dir = env!("CARGO_MANIFEST_DIR");
    std::path::Path::new(project_dir)
        .join("tests")
        .join("set_program_metadata_visitor")
        .join("fixtures")
        .join(relative_path)
}
