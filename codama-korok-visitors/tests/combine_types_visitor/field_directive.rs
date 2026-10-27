use codama_errors::CodamaResult;
use codama_korok_visitors::{CombineTypesVisitor, KorokVisitable};
use codama_koroks::ItemKorok;

#[test]
fn it_rejects_field_directives_on_tuples() -> CodamaResult<()> {
    for item in [
        syn::parse_quote! {
            #[codama(field("age", integer(u8)))]
            struct Person(String, u64);
        },
        syn::parse_quote! {
            enum Message {
                #[codama(field(after, "is_member", boolean))]
                Person(String, u64),
            }
        },
    ] {
        let mut korok = ItemKorok::parse(&item, &[], &mut 0)?;
        let error = korok.accept(&mut CombineTypesVisitor::new()).unwrap_err();
        assert_eq!(
            error.to_string(),
            "`field(...)` needs named fields, it cannot add a field to a tuple"
        );
    }
    Ok(())
}
