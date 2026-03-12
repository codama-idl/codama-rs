use codama_errors::CodamaResult;
use codama_korok_visitors::{
    CombineModulesVisitor, IdentifyFieldTypesVisitor, KorokVisitable, SetAccountsVisitor,
    SetInstructionsVisitor, SetProgramMetadataVisitor,
};
use codama_koroks::CrateKorok;
use codama_nodes::Node;
use codama_stores::CrateStore;
use quote::quote;

#[test]
fn it_separates_items_with_program_directive_from_main_program() -> CodamaResult<()> {
    let store = CrateStore::hydrate(quote! {
        solana_program::declare_id!("MainProg1111111111111111111111111111111111111");

        mod my_module {
            #[derive(CodamaAccount)]
            struct MyAccount {
                owner: Pubkey,
            }

            #[derive(CodamaAccount)]
            #[codama(program(name = "externalProgram", address = "ExtProg111111111111111111111111111111111111"))]
            struct ExternalAccount {
                amount: u64,
            }
        }
    })?;
    let mut korok = CrateKorok::parse(&store)?;

    korok.accept(&mut SetProgramMetadataVisitor::new())?;
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetAccountsVisitor::new())?;
    korok.accept(&mut CombineModulesVisitor::new())?;

    let Some(Node::Root(root)) = &korok.node else {
        panic!("Expected a RootNode");
    };

    // MyAccount belongs to the main program.
    assert_eq!(
        root.program.public_key,
        "MainProg1111111111111111111111111111111111111"
    );
    assert_eq!(root.program.accounts.len(), 1);
    assert_eq!(root.program.accounts[0].name, "myAccount".into());

    // ExternalAccount is in an additional program.
    assert_eq!(root.additional_programs.len(), 1);
    assert_eq!(root.additional_programs[0].name, "externalProgram".into());
    assert_eq!(
        root.additional_programs[0].public_key,
        "ExtProg111111111111111111111111111111111111"
    );
    assert_eq!(root.additional_programs[0].accounts.len(), 1);
    assert_eq!(
        root.additional_programs[0].accounts[0].name,
        "externalAccount".into()
    );

    Ok(())
}

#[test]
fn it_separates_items_with_program_directive_regardless_of_order() -> CodamaResult<()> {
    let store = CrateStore::hydrate(quote! {
        solana_program::declare_id!("MainProg1111111111111111111111111111111111111");

        mod my_module {
            #[derive(CodamaAccount)]
            #[codama(program(name = "externalProgram", address = "ExtProg111111111111111111111111111111111111"))]
            struct ExternalAccount {
                amount: u64,
            }

            #[derive(CodamaAccount)]
            struct MyAccount {
                owner: Pubkey,
            }
        }
    })?;
    let mut korok = CrateKorok::parse(&store)?;

    korok.accept(&mut SetProgramMetadataVisitor::new())?;
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetAccountsVisitor::new())?;
    korok.accept(&mut CombineModulesVisitor::new())?;

    let Some(Node::Root(root)) = &korok.node else {
        panic!("Expected a RootNode");
    };

    // MyAccount belongs to the main program.
    assert_eq!(
        root.program.public_key,
        "MainProg1111111111111111111111111111111111111"
    );
    assert_eq!(root.program.accounts.len(), 1);
    assert_eq!(root.program.accounts[0].name, "myAccount".into());

    // ExternalAccount is in an additional program.
    assert_eq!(root.additional_programs.len(), 1);
    assert_eq!(root.additional_programs[0].name, "externalProgram".into());
    assert_eq!(
        root.additional_programs[0].public_key,
        "ExtProg111111111111111111111111111111111111"
    );
    assert_eq!(root.additional_programs[0].accounts.len(), 1);
    assert_eq!(
        root.additional_programs[0].accounts[0].name,
        "externalAccount".into()
    );

    Ok(())
}

#[test]
fn it_merges_multiple_items_with_same_program_directive() -> CodamaResult<()> {
    let store = CrateStore::hydrate(quote! {
        solana_program::declare_id!("MainProg1111111111111111111111111111111111111");

        mod my_module {
            #[derive(CodamaAccount)]
            #[codama(program(name = "externalProgram", address = "ExtProg111111111111111111111111111111111111"))]
            struct ExternalAccount {
                owner: Pubkey,
            }

            #[derive(CodamaInstruction)]
            #[codama(program(name = "externalProgram", address = "ExtProg111111111111111111111111111111111111"))]
            struct ExternalInstruction {
                amount: u64,
            }
        }
    })?;
    let mut korok = CrateKorok::parse(&store)?;

    korok.accept(&mut SetProgramMetadataVisitor::new())?;
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetAccountsVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    korok.accept(&mut CombineModulesVisitor::new())?;

    let Some(Node::Root(root)) = &korok.node else {
        panic!("Expected a RootNode");
    };

    // The main program has no items but holds the crate's public key.
    assert_eq!(
        root.program.public_key,
        "MainProg1111111111111111111111111111111111111"
    );
    assert_eq!(root.program.accounts.len(), 0);

    // Both items share the same address, so they merge into a single additional program.
    assert_eq!(root.additional_programs.len(), 1);
    assert_eq!(root.additional_programs[0].name, "externalProgram".into());
    assert_eq!(
        root.additional_programs[0].public_key,
        "ExtProg111111111111111111111111111111111111"
    );
    assert_eq!(root.additional_programs[0].accounts.len(), 1);
    assert_eq!(root.additional_programs[0].instructions.len(), 1);

    Ok(())
}

#[test]
fn it_applies_program_directive_on_mod_block() -> CodamaResult<()> {
    let store = CrateStore::hydrate(quote! {
        solana_program::declare_id!("MainProg1111111111111111111111111111111111111");

        mod parent_module {
            mod my_program {
                #[derive(CodamaAccount)]
                struct MyAccount {
                    owner: Pubkey,
                }

                #[derive(CodamaInstruction)]
                struct MyInstruction {
                    amount: u64,
                }
            }

            #[codama(program(name = "externalProgram", address = "ExtProg111111111111111111111111111111111111"))]
            mod my_external_program {
                #[derive(CodamaAccount)]
                struct SomeExternalAccount {
                    balance: u64,
                }

                #[derive(CodamaInstruction)]
                struct SomeExternalInstruction {
                    data: u64,
                }
            }
        }
    })?;
    let mut korok = CrateKorok::parse(&store)?;

    korok.accept(&mut SetProgramMetadataVisitor::new())?;
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetAccountsVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    korok.accept(&mut CombineModulesVisitor::new())?;

    let Some(Node::Root(root)) = &korok.node else {
        panic!("Expected a RootNode");
    };

    // The main program should contain items from my_program.
    assert_eq!(
        root.program.public_key,
        "MainProg1111111111111111111111111111111111111"
    );
    assert_eq!(root.program.accounts.len(), 1);
    assert_eq!(root.program.accounts[0].name, "myAccount".into());
    assert_eq!(root.program.instructions.len(), 1);
    assert_eq!(root.program.instructions[0].name, "myInstruction".into());

    // The external program should be an additional program with its items.
    assert_eq!(root.additional_programs.len(), 1);
    let ext = &root.additional_programs[0];
    assert_eq!(ext.name, "externalProgram".into());
    assert_eq!(
        ext.public_key,
        "ExtProg111111111111111111111111111111111111"
    );
    assert_eq!(ext.accounts.len(), 1);
    assert_eq!(ext.accounts[0].name, "someExternalAccount".into());
    assert_eq!(ext.instructions.len(), 1);
    assert_eq!(ext.instructions[0].name, "someExternalInstruction".into());

    Ok(())
}
