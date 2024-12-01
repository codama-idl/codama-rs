use codama_stores::{CrateStore, RootStore};
use quote::quote;

#[test]
fn root_from_tokens() {
    let tt = quote! {
        enum Membership { None, Basic, Premium }
        struct Person {
            name: String,
            age: u8,
            member: Membership,
        }
    };

    let store = RootStore::populate_from(tt).unwrap();
    assert_eq!(store.crates.len(), 1);
    assert!(matches!(store.crates[0].file, syn::File { .. }));
}

#[test]
fn crate_from_tokens() {
    let tt = quote! {
        enum Membership { None, Basic, Premium }
        struct Person {
            name: String,
            age: u8,
            member: Membership,
        }
    };

    let store = CrateStore::populate_from(tt).unwrap();
    assert!(matches!(store.file, syn::File { .. }));
    assert!(matches!(store.manifest, None));
    assert_eq!(store.file_modules.len(), 0);
    assert_eq!(store.path.to_str(), Some(""));
}
