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

#[test]
fn to_compile_error_keeps_errors_without_a_span() {
    // A derive that fails with a plain error must still emit a diagnostic, rather than nothing.
    let error = CodamaError::NodeNotFound;
    let tokens = error.to_compile_error().to_string();
    assert!(tokens.contains("compile_error"), "{tokens}");
    assert!(tokens.contains("\"Node not found\""), "{tokens}");
}
