use crate::get_path;
use codama_koroks::{ItemKorok, RootKorok};
use codama_stores::RootStore;

#[test]
fn load_single_crate() {
    let path = get_path("single_crate/crate");
    let root_store: RootStore = RootStore::load_all(&[&path]).unwrap();
    let root_korok = RootKorok::parse(&root_store).unwrap();

    // Check the root korok.
    let [crate_korok] = &root_korok.crates.as_slice() else {
        panic!("Unexpected number of crates");
    };

    // Check the crate korok.
    assert_eq!(
        crate_korok.store.path,
        get_path("single_crate/crate/src/lib.rs")
    );
    let [ItemKorok::FileModule(membership), ItemKorok::FileModule(person), ItemKorok::Unsupported(_), ItemKorok::Unsupported(_)] =
        &crate_korok.items.as_slice()
    else {
        panic!("Unexpected items in lib.rs");
    };

    // Check the membership module.
    assert_eq!(
        membership.store.path,
        get_path("single_crate/crate/src/membership.rs")
    );
    let [ItemKorok::Enum(_)] = &membership.items.as_slice() else {
        panic!("Unexpected items in membership.rs");
    };

    // Check the person module.
    assert_eq!(
        person.store.path,
        get_path("single_crate/crate/src/person.rs")
    );
    let [ItemKorok::Unsupported(_), ItemKorok::Struct(_)] = &person.items.as_slice() else {
        panic!("Unexpected items in person.rs");
    };
}
