use codama_errors::CodamaResult;
use codama_korok_visitors::{
    CombineTypesVisitor, IdentifyFieldTypesVisitor, KorokVisitable, KorokVisitor,
    SetAccountsVisitor, SetDefinedTypesVisitor, SetErrorsVisitor, SetEventsVisitor,
};
use codama_koroks::ItemKorok;

#[test]
fn it_rejects_contextual_default_values_on_fields() -> CodamaResult<()> {
    for item in [
        syn::parse_quote! {
            struct Person {
                #[codama(default_value = account_bump("vault"))]
                bump: u8,
            }
        },
        syn::parse_quote! {
            struct Person(#[codama(default_value = account_bump("vault"))] u8);
        },
        syn::parse_quote! {
            enum Message {
                Person {
                    #[codama(value = account_bump("vault"))]
                    bump: u8,
                },
            }
        },
    ] {
        let mut korok = ItemKorok::parse(&item, &[], &mut 0)?;
        korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
        let error = korok.accept(&mut CombineTypesVisitor::new()).unwrap_err();
        assert_eq!(
            error.to_string(),
            "`accountBumpValueNode` is a contextual value, only instruction fields can default to it"
        );
    }
    Ok(())
}

#[test]
fn it_rejects_contextual_default_values_outside_instructions() -> CodamaResult<()> {
    let visitors: [(syn::Item, Box<dyn KorokVisitor>); 4] = [
        (
            syn::parse_quote! {
                #[derive(CodamaType)]
                struct Config {
                    #[codama(default_value = payer)]
                    owner: Pubkey,
                }
            },
            Box::new(SetDefinedTypesVisitor::new()),
        ),
        (
            syn::parse_quote! {
                #[derive(CodamaAccount)]
                struct Vault {
                    #[codama(default_value = payer)]
                    owner: Pubkey,
                }
            },
            Box::new(SetAccountsVisitor::new()),
        ),
        (
            syn::parse_quote! {
                #[derive(CodamaEvent)]
                struct Deposited {
                    #[codama(default_value = payer)]
                    owner: Pubkey,
                }
            },
            Box::new(SetEventsVisitor::new()),
        ),
        (
            syn::parse_quote! {
                #[derive(CodamaErrors)]
                enum Error {
                    #[codama(error(code = 1, message = "Invalid owner"))]
                    InvalidOwner {
                        #[codama(default_value = payer)]
                        owner: Pubkey,
                    },
                }
            },
            Box::new(SetErrorsVisitor::new()),
        ),
    ];
    for (item, mut visitor) in visitors {
        let mut korok = ItemKorok::parse(&item, &[], &mut 0)?;
        korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
        let error = korok.accept(visitor.as_mut()).unwrap_err();
        assert_eq!(
            error.to_string(),
            "`payerValueNode` is a contextual value, only instruction fields can default to it"
        );
    }
    Ok(())
}
