use codama_errors::CodamaResult;
use codama_korok_visitors::{
    CombineModulesVisitor, IdentifyFieldTypesVisitor, KorokVisitable, SetDefinedTypesVisitor,
    SetProgramMetadataVisitor,
};
use codama_koroks::CrateKorok;
use codama_nodes::Node;
use codama_stores::CrateStore;
use quote::quote;

#[test]
fn it_fails_when_two_modules_define_different_types_with_the_same_name() -> CodamaResult<()> {
    let store = CrateStore::hydrate(quote! {
        solana_program::declare_id!("MainProg1111111111111111111111111111111111111");

        mod module_a {
            #[derive(CodamaType)]
            struct Config {
                amount: u8,
            }
        }

        mod module_b {
            #[derive(CodamaType)]
            struct Config {
                amount: u16,
            }
        }
    })?;
    let mut korok = CrateKorok::parse(&store)?;

    korok.accept(&mut SetProgramMetadataVisitor::new())?;
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetDefinedTypesVisitor::new())?;
    let error = korok.accept(&mut CombineModulesVisitor::new()).unwrap_err();

    assert_eq!(
        error.to_string(),
        "Two different `definedTypeNode` share the identifier `config` in program `MainProg1111111111111111111111111111111111111`"
    );
    Ok(())
}

#[test]
fn it_keeps_one_type_when_two_modules_define_it_identically() -> CodamaResult<()> {
    let store = CrateStore::hydrate(quote! {
        mod module_a {
            #[derive(CodamaType)]
            struct Config {
                amount: u8,
            }
        }

        mod module_b {
            #[derive(CodamaType)]
            struct Config {
                amount: u8,
            }
        }
    })?;
    let mut korok = CrateKorok::parse(&store)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetDefinedTypesVisitor::new())?;
    korok.accept(&mut CombineModulesVisitor::new())?;

    let Some(Node::Root(root)) = &korok.node else {
        panic!("Expected a RootNode");
    };
    assert_eq!(root.program.defined_types.len(), 1);
    assert_eq!(root.program.defined_types[0].identifier, "config".into());
    Ok(())
}

#[test]
fn it_names_the_module_when_two_of_its_submodules_clash() -> CodamaResult<()> {
    let store = CrateStore::hydrate(quote! {
        mod parent {
            mod module_a {
                #[derive(CodamaType)]
                struct Config {
                    amount: u8,
                }
            }

            mod module_b {
                #[derive(CodamaType)]
                struct Config {
                    amount: u16,
                }
            }
        }
    })?;
    let mut korok = CrateKorok::parse(&store)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetDefinedTypesVisitor::new())?;
    let error = korok.accept(&mut CombineModulesVisitor::new()).unwrap_err();

    assert_eq!(
        error.to_string(),
        "Two different `definedTypeNode` share the identifier `config` in module `parent`"
    );
    Ok(())
}

#[test]
fn it_names_the_crate_root_when_two_of_its_items_clash() -> CodamaResult<()> {
    let store = CrateStore::hydrate(quote! {
        #[derive(CodamaType)]
        struct Config {
            amount: u8,
        }

        #[derive(CodamaType)]
        struct Config {
            amount: u16,
        }
    })?;
    let mut korok = CrateKorok::parse(&store)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetDefinedTypesVisitor::new())?;
    let error = korok.accept(&mut CombineModulesVisitor::new()).unwrap_err();

    assert_eq!(
        error.to_string(),
        "Two different `definedTypeNode` share the identifier `config` in the crate root"
    );
    Ok(())
}
