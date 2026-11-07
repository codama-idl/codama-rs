use codama::{
    Codama, CodamaError, CodamaResult, DefinedTypeNode, KorokPlugin, KorokVisitable, KorokVisitor,
    Node, ProgramNode, RootKorok, RootNode, StructFieldTypeNode, StructTypeNode, TypeNode,
};
use codama_nodes::test_utils::id;
use proc_macro2::TokenStream;
use quote::quote;

fn hydrate(item: TokenStream) -> Codama {
    Codama::hydrate(quote! {
        codama_program!(identifier = "token");
        #item
    })
    .unwrap()
}

fn errors(codama: Codama) -> Vec<String> {
    match codama.get_idl() {
        Err(CodamaError::InvalidRootNode(errors)) => errors,
        other => panic!("expected an invalid root node, got {other:?}"),
    }
}

struct OnRootNodeSet(fn(&mut RootNode));

impl KorokPlugin for OnRootNodeSet {
    fn on_root_node_set(&self, visitable: &mut dyn KorokVisitable) -> CodamaResult<()> {
        visitable.accept(&mut UpdateRootVisitor(self.0))
    }
}

struct UpdateRootVisitor(fn(&mut RootNode));

impl KorokVisitor for UpdateRootVisitor {
    fn visit_root(&mut self, korok: &mut RootKorok) -> CodamaResult<()> {
        if let Node::Root(root) = korok.node.get_or_insert_with(|| RootNode::default().into()) {
            (self.0)(root);
        }
        Ok(())
    }
}

fn config_fields(root: &mut RootNode) -> &mut Vec<StructFieldTypeNode> {
    match root.program.defined_types[0].r#type.as_mut() {
        TypeNode::Struct(node) => &mut node.fields,
        other => panic!("expected a struct type, got {other:?}"),
    }
}

#[test]
fn it_rejects_rust_fields_with_one_identifier() {
    let transfer_data = "program \"token\" > instruction \"transfer\" > data";
    let transfer_provides = "Provided value identifier \"owner\" is not unique in program \"token\" > instruction \"transfer\".";
    let deposited_data = "program \"token\" > event \"deposited\" > data";
    let cases: [(TokenStream, Vec<String>); 11] = [
        (
            quote! {
                #[derive(CodamaInstruction)]
                struct Transfer {
                    #[codama(identifier = "owner")]
                    #[codama(default_value = payer)]
                    source_owner: Pubkey,
                    #[codama(identifier = "owner")]
                    #[codama(default_value = payer)]
                    destination_owner: Pubkey,
                }
            },
            vec![
                transfer_provides.into(),
                format!("Struct field identifier \"owner\" is not unique in {transfer_data}."),
            ],
        ),
        (
            quote! {
                #[derive(CodamaInstruction)]
                #[codama(field("owner", public_key, default_value = payer))]
                struct Transfer {
                    #[codama(default_value = payer)]
                    owner: Pubkey,
                }
            },
            vec![
                transfer_provides.into(),
                format!("Struct field identifier \"owner\" is not unique in {transfer_data}."),
            ],
        ),
        (
            quote! {
                #[derive(CodamaInstruction)]
                #[codama(field("owner", public_key, default_value = payer))]
                #[codama(field(after, "owner", public_key, default_value = payer))]
                struct Transfer {
                    amount: u64,
                }
            },
            vec![
                transfer_provides.into(),
                format!("Struct field identifier \"owner\" is not unique in {transfer_data}."),
            ],
        ),
        (
            quote! {
                #[derive(CodamaType)]
                struct Config {
                    owner_key: Pubkey,
                    #[codama(identifier = "OwnerKey")]
                    key: Pubkey,
                }
            },
            vec!["Struct field identifier \"OwnerKey\" collides with \"owner_key\" in program \"token\" > definedType \"config\" > type.".into()],
        ),
        (
            quote! {
                #[derive(CodamaEvents)]
                enum Events {
                    #[codama(field(after, "amount", integer(u64)))]
                    Deposited { amount: u64 },
                }
            },
            vec![format!("Struct field identifier \"amount\" is not unique in {deposited_data}.")],
        ),
        (
            quote! {
                #[derive(CodamaInstructions)]
                enum Instructions {
                    Transfer(#[codama(identifier = "amount")] u64, #[codama(identifier = "amount")] u64),
                }
            },
            vec![format!("Struct field identifier \"amount\" is not unique in {transfer_data}.")],
        ),
        (
            quote! {
                #[derive(CodamaInstructions)]
                enum Instructions {
                    Transfer(#[codama(identifier = "arg1")] u64, u64),
                }
            },
            vec![format!("Struct field identifier \"arg1\" is not unique in {transfer_data}.")],
        ),
        (
            quote! {
                #[derive(CodamaInstructions)]
                enum Instructions {
                    Transfer { discriminator: u8 },
                }
            },
            vec![format!("Struct field identifier \"discriminator\" is not unique in {transfer_data}.")],
        ),
        (
            quote! {
                #[derive(CodamaEvents)]
                enum Events {
                    Deposited(#[codama(identifier = "amount")] u64, #[codama(identifier = "amount")] u64),
                }
            },
            vec![format!("Struct field identifier \"amount\" is not unique in {deposited_data}.")],
        ),
        (
            quote! {
                #[derive(CodamaEvents)]
                enum Events {
                    Deposited { discriminator: u8 },
                }
            },
            vec![format!("Struct field identifier \"discriminator\" is not unique in {deposited_data}.")],
        ),
        (
            quote! {
                #[derive(CodamaAccounts)]
                enum Accounts {
                    Vault { discriminator: u8 },
                }
            },
            vec!["Struct field identifier \"discriminator\" is not unique in program \"token\" > account \"vault\" > data.".into()],
        ),
    ];
    for (item, expected) in cases {
        assert_eq!(errors(hydrate(item)), expected);
    }
}

#[test]
fn it_reports_every_error_in_one_error() {
    let codama = hydrate(quote! {
        #[derive(CodamaType)]
        struct Config {
            #[codama(identifier = "owner")]
            first: Pubkey,
            #[codama(identifier = "owner")]
            second: Pubkey,
        }

        #[derive(CodamaType)]
        enum Command {
            #[codama(identifier = "quit")]
            Exit,
            Quit,
        }
    });
    assert_eq!(
        codama.get_idl().unwrap_err().to_string(),
        "Struct field identifier \"owner\" is not unique in program \"token\" > definedType \"config\" > type.\n\
         Enum variant identifier \"quit\" is not unique in program \"token\" > definedType \"command\" > type."
    );
}

#[test]
fn it_rejects_a_program_without_an_identifier() {
    let codama = Codama::hydrate(quote! {}).unwrap();
    assert_eq!(
        errors(codama),
        ["The program has no identifier. Set one with `codama_program!(identifier = \"...\")` or `#[codama(program(identifier = \"...\", address = \"...\"))]`."]
    );
}

#[test]
fn it_rejects_a_link_to_a_missing_node() {
    let account_with_pda_link = |pda: &str| {
        hydrate(quote! {
            #[derive(CodamaPda)]
            #[codama(seed(identifier = "authority", type = public_key))]
            struct CounterPda;

            #[derive(CodamaAccount)]
            #[codama(pda = #pda)]
            struct Counter;
        })
    };
    assert!(account_with_pda_link("counter_pda").get_idl().is_ok());
    assert_eq!(
        errors(account_with_pda_link("counterPda")),
        ["Pointing to a missing PDA named \"counterPda\" in program \"token\" > account \"counter\" > pdaLink \"counterPda\"."]
    );
}

#[test]
fn it_accepts_enum_accounts_of_another_program_that_share_a_pda() {
    let idl = hydrate(quote! {
        #[derive(CodamaAccounts)]
        #[codama(program(identifier = "vaults", address = "Vau1t11111111111111111111111111111111111111"))]
        enum Accounts {
            #[codama(pda = "vault")]
            #[codama(seed(identifier = "owner", type = public_key))]
            First,
            #[codama(pda = "vault")]
            #[codama(seed(identifier = "owner", type = public_key))]
            Second,
        }
    })
    .get_idl()
    .unwrap();
    let pdas: Vec<_> = idl.additional_programs[0]
        .pdas
        .iter()
        .map(|pda| pda.identifier.clone())
        .collect();
    assert_eq!(pdas, [id("vault")]);
}

#[test]
fn it_validates_after_the_root_node_hooks_of_all_plugins() {
    let duplicates = quote! {
        #[derive(CodamaType)]
        struct Config {
            #[codama(identifier = "owner")]
            first: Pubkey,
            #[codama(identifier = "owner")]
            second: Pubkey,
        }
    };
    let codama = hydrate(duplicates).add_plugin(OnRootNodeSet(|root| {
        config_fields(root)[1].identifier = id("new_owner");
    }));
    let mut idl = codama.get_idl().unwrap();
    let identifiers: Vec<_> = config_fields(&mut idl)
        .iter()
        .map(|f| f.identifier.clone())
        .collect();
    assert_eq!(identifiers, [id("owner"), id("new_owner")]);

    let unique = quote! {
        #[derive(CodamaType)]
        struct Config {
            owner: Pubkey,
        }
    };
    let codama = hydrate(unique).add_plugin(OnRootNodeSet(|root| {
        let owner = config_fields(root)[0].clone();
        config_fields(root).push(owner);
    }));
    assert_eq!(
        errors(codama),
        ["Struct field identifier \"owner\" is not unique in program \"token\" > definedType \"config\" > type."]
    );
}

#[test]
fn it_validates_without_the_default_plugin() {
    let codama = Codama::hydrate(quote! {})
        .unwrap()
        .without_default_plugin()
        .add_plugin(OnRootNodeSet(|root| {
            let config = DefinedTypeNode::new(id("config"), StructTypeNode::new(vec![]));
            root.program = ProgramNode::new(id("token"), "1111")
                .add_defined_type(config.clone())
                .add_defined_type(config);
        }));
    assert_eq!(
        errors(codama),
        ["Defined type identifier \"config\" is not unique in program \"token\"."]
    );
}
