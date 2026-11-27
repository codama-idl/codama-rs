//! Duplicate field identifiers are reported by the validator on the final root node, once per
//! duplicate and against its path, rather than by the visitors building the node.

use codama_errors::CodamaResult;
use codama_korok_visitors::{
    CombineModulesVisitor, IdentifyFieldTypesVisitor, KorokVisitable, SetAccountsVisitor,
    SetDefinedTypesVisitor, SetEventsVisitor, SetInstructionsVisitor, SetProgramMetadataVisitor,
};
use codama_koroks::RootKorok;
use codama_nodes::Node;
use codama_stores::RootStore;
use codama_validators::{get_validation_items_visitor, LogLevel};
use codama_visitors_core::visit;
use proc_macro2::TokenStream;
use quote::quote;

/// The error-level validation items of the root node built from `tt`, as (message, path) pairs.
fn errors(tt: TokenStream) -> CodamaResult<Vec<(String, String)>> {
    let store = RootStore::hydrate(quote! {
        codama_program!(identifier = "dup_check", address = "Dup1111111111111111111111111111111111111111");
        #tt
    })?;
    let mut korok = RootKorok::parse(&store)?;
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetDefinedTypesVisitor::new())?;
    korok.accept(&mut SetAccountsVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    korok.accept(&mut SetEventsVisitor::new())?;
    korok.accept(&mut SetProgramMetadataVisitor::new())?;
    korok.accept(&mut CombineModulesVisitor::new())?;
    let Some(Node::Root(root)) = korok.node else {
        panic!("Expected a root node");
    };
    Ok(visit(&get_validation_items_visitor(), &root)
        .into_iter()
        .filter(|item| item.level == LogLevel::Error)
        .map(|item| (item.message, item.path.to_string()))
        .collect())
}

const PROGRAM: &str = "[programNode]dup_check";

fn field(owner: &str, field: &str) -> String {
    format!("{PROGRAM}.{owner}.[structTypeNode].[structFieldTypeNode]{field}")
}

#[test]
fn it_reports_fields_sharing_an_identifier_once_against_their_path() -> CodamaResult<()> {
    let transfer = "[instructionNode]transfer";
    let deposited = "[eventNode]deposited";
    let vault = "[accountNode]vault";
    let not_unique =
        |identifier: &str| format!("Struct field identifier \"{identifier}\" is not unique.");
    let cases: [(TokenStream, Vec<(String, String)>); 11] = [
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
            // The contextual defaults are provided under the field identifiers, which collide too.
            vec![
                (
                    "Provided value identifier \"owner\" is not unique in instruction \"transfer\"."
                        .to_string(),
                    format!("{PROGRAM}.{transfer}.[providedNode]owner"),
                ),
                (not_unique("owner"), field(transfer, "owner")),
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
                (
                    "Provided value identifier \"owner\" is not unique in instruction \"transfer\"."
                        .to_string(),
                    format!("{PROGRAM}.{transfer}.[providedNode]owner"),
                ),
                (not_unique("owner"), field(transfer, "owner")),
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
            vec![(
                "Struct field identifier \"OwnerKey\" collides with \"owner_key\" once converted to camelCase."
                    .to_string(),
                field("[definedTypeNode]config", "OwnerKey"),
            )],
        ),
        (
            quote! {
                #[derive(CodamaEvents)]
                enum Events {
                    #[codama(field(after, "amount", integer(u64)))]
                    Deposited { amount: u64 },
                }
            },
            vec![(not_unique("amount"), field(deposited, "amount"))],
        ),
        (
            quote! {
                #[derive(CodamaInstructions)]
                enum Instructions {
                    Transfer(#[codama(identifier = "amount")] u64, #[codama(identifier = "amount")] u64),
                }
            },
            vec![(not_unique("amount"), field(transfer, "amount"))],
        ),
        (
            quote! {
                #[derive(CodamaInstructions)]
                enum Instructions {
                    Transfer(#[codama(identifier = "arg1")] u64, u64),
                }
            },
            vec![(not_unique("arg1"), field(transfer, "arg1"))],
        ),
        (
            quote! {
                #[derive(CodamaInstructions)]
                enum Instructions {
                    Transfer { discriminator: u8 },
                }
            },
            vec![(not_unique("discriminator"), field(transfer, "discriminator"))],
        ),
        (
            quote! {
                #[derive(CodamaEvents)]
                enum Events {
                    Deposited(#[codama(identifier = "amount")] u64, #[codama(identifier = "amount")] u64),
                }
            },
            vec![(not_unique("amount"), field(deposited, "amount"))],
        ),
        (
            quote! {
                #[derive(CodamaEvents)]
                enum Events {
                    Deposited { discriminator: u8 },
                }
            },
            vec![(not_unique("discriminator"), field(deposited, "discriminator"))],
        ),
        (
            quote! {
                #[derive(CodamaAccounts)]
                enum Accounts {
                    Vault { discriminator: u8 },
                }
            },
            vec![(not_unique("discriminator"), field(vault, "discriminator"))],
        ),
        (
            quote! {
                #[derive(CodamaAccounts)]
                enum Accounts {
                    Vault(#[codama(identifier = "owner")] Pubkey, #[codama(identifier = "owner")] Pubkey),
                }
            },
            vec![(not_unique("owner"), field(vault, "owner"))],
        ),
    ];
    for (tt, expected) in cases {
        assert_eq!(errors(tt.clone())?, expected, "{tt}");
    }
    Ok(())
}
