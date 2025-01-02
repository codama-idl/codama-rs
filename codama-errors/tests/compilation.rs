use codama_errors::CodamaError;
use proc_macro2::TokenStream;

#[test]
fn from_syn_error() {
    let error: CodamaError =
        syn::Error::new_spanned(TokenStream::new(), "Could not parse Rust code").into();
    assert!(matches!(error, CodamaError::Compilation(_)));
}

#[test]
fn display() {
    let error: CodamaError =
        syn::Error::new_spanned(TokenStream::new(), "Could not parse Rust code").into();
    assert_eq!(error.to_string(), "Could not parse Rust code");
}
