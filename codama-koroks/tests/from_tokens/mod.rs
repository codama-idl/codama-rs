use codama_koroks::{CrateKorok, ItemKorok, RootKorok};
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

    let store = RootStore::hydrate(tt).unwrap();
    let korok = RootKorok::parse(&store).unwrap();
    assert_eq!(korok.store, &store);
    assert_eq!(korok.crates.len(), 1);
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

    let store = CrateStore::hydrate(tt).unwrap();
    let korok = CrateKorok::parse(&store).unwrap();
    assert_eq!(korok.store, &store);
    assert_eq!(korok.node, None);
    assert!(matches!(
        korok.items.as_slice(),
        [ItemKorok::Enum(_), ItemKorok::Struct(_),]
    ));
}
