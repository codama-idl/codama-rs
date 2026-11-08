use crate::get_path;
use codama::Codama;

/// A Rust name with no identifier, such as a non-ASCII field, fails with the cause and the fix.
#[test]
fn it_explains_why_a_rust_name_has_no_identifier() {
    let codama = Codama::load(get_path("rust_names/crate")).unwrap();
    let error = codama.get_idl().unwrap_err();
    assert_eq!(
        error.to_string(),
        "Cannot derive an identifier from `été`: identifiers only take ASCII letters. Set one explicitly with `identifier = \"...\"`"
    );
}
