use codama_errors::CodamaResult;
use codama_korok_visitors::{
    IdentifyFieldTypesVisitor, KorokVisitable, KorokVisitor, SetAccountsVisitor,
    SetDefinedTypesVisitor, SetEventsVisitor, SetInstructionsVisitor,
};
use codama_koroks::ItemKorok;

#[test]
fn it_rejects_fields_with_one_identifier() -> CodamaResult<()> {
    let cases: [(syn::Item, Box<dyn KorokVisitor>, &str); 11] = [
        (
            syn::parse_quote! {
                #[derive(CodamaInstruction)]
                struct Transfer {
                    #[codama(name = "owner")]
                    #[codama(default_value = payer)]
                    source_owner: Pubkey,
                    #[codama(name = "owner")]
                    #[codama(default_value = payer)]
                    destination_owner: Pubkey,
                }
            },
            Box::new(SetInstructionsVisitor::new()),
            "Field `owner` is not unique in struct `Transfer`",
        ),
        (
            syn::parse_quote! {
                #[derive(CodamaInstruction)]
                #[codama(field("owner", public_key, default_value = payer))]
                struct Transfer {
                    #[codama(default_value = account("authority"))]
                    owner: Pubkey,
                }
            },
            Box::new(SetInstructionsVisitor::new()),
            "Field `owner` is not unique in struct `Transfer`",
        ),
        (
            syn::parse_quote! {
                #[derive(CodamaInstruction)]
                #[codama(field("owner", public_key, default_value = payer))]
                #[codama(field(after, "owner", public_key, default_value = account("authority")))]
                struct Transfer {
                    amount: u64,
                }
            },
            Box::new(SetInstructionsVisitor::new()),
            "Field `owner` is not unique in struct `Transfer`",
        ),
        (
            syn::parse_quote! {
                #[derive(CodamaType)]
                struct Config {
                    owner_key: Pubkey,
                    #[codama(name = "OwnerKey")]
                    key: Pubkey,
                }
            },
            Box::new(SetDefinedTypesVisitor::new()),
            "Field `ownerKey` is not unique in struct `Config`",
        ),
        (
            syn::parse_quote! {
                #[derive(CodamaEvents)]
                enum Events {
                    #[codama(field(after, "amount", integer(u64)))]
                    Deposited { amount: u64 },
                }
            },
            Box::new(SetEventsVisitor::new()),
            "Field `amount` is not unique in variant `Deposited` of enum `Events`",
        ),
        (
            syn::parse_quote! {
                #[derive(CodamaInstructions)]
                enum Instructions {
                    Transfer(#[codama(name = "amount")] u64, #[codama(name = "amount")] u64),
                }
            },
            Box::new(SetInstructionsVisitor::new()),
            "Field `amount` is not unique in variant `Transfer` of enum `Instructions`",
        ),
        (
            syn::parse_quote! {
                #[derive(CodamaInstructions)]
                enum Instructions {
                    Transfer(#[codama(name = "arg1")] u64, u64),
                }
            },
            Box::new(SetInstructionsVisitor::new()),
            "Field `arg1` is not unique in variant `Transfer` of enum `Instructions`",
        ),
        (
            syn::parse_quote! {
                #[derive(CodamaInstructions)]
                enum Instructions {
                    Transfer { discriminator: u8 },
                }
            },
            Box::new(SetInstructionsVisitor::new()),
            "Field `discriminator` is not unique in variant `Transfer` of enum `Instructions`",
        ),
        (
            syn::parse_quote! {
                #[derive(CodamaEvents)]
                enum Events {
                    Deposited(#[codama(name = "amount")] u64, #[codama(name = "amount")] u64),
                }
            },
            Box::new(SetEventsVisitor::new()),
            "Field `amount` is not unique in variant `Deposited` of enum `Events`",
        ),
        (
            syn::parse_quote! {
                #[derive(CodamaEvents)]
                enum Events {
                    Deposited { discriminator: u8 },
                }
            },
            Box::new(SetEventsVisitor::new()),
            "Field `discriminator` is not unique in variant `Deposited` of enum `Events`",
        ),
        (
            syn::parse_quote! {
                #[derive(CodamaAccounts)]
                enum Accounts {
                    Vault { discriminator: u8 },
                }
            },
            Box::new(SetAccountsVisitor::new()),
            "Field `discriminator` is not unique in variant `Vault` of enum `Accounts`",
        ),
    ];
    for (item, mut visitor, expected) in cases {
        let mut korok = ItemKorok::parse(&item, &[], &mut 0)?;
        korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
        let error = korok.accept(visitor.as_mut()).unwrap_err();
        assert_eq!(error.to_string(), expected);
    }
    Ok(())
}
