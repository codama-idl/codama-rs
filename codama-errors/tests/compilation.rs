use codama_errors::CodamaError;

#[test]
fn from_syn_error() {
    let error: CodamaError =
        syn::Error::new_spanned(quote::quote! {}, "Could not parse Rust code").into();
    assert!(matches!(error, CodamaError::Compilation(_)));
}

#[test]
fn display() {
    let error: CodamaError =
        syn::Error::new_spanned(quote::quote! {}, "Could not parse Rust code").into();
    assert_eq!(
        error.to_string(),
        "A compilation error was identified via Syn: Could not parse Rust code"
    );
}
