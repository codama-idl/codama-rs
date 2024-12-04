use crate::get_path;
use codama_koroks::{ItemKorok, RootKorok};
use codama_stores::RootStore;

#[test]
fn load_nested_modules() {
    let path = get_path("nested_modules/crate");
    let root_store = RootStore::load_from(&[&path]).unwrap();
    let root_korok = RootKorok::parse(&root_store).unwrap();

    // Check the root korok.
    let [crate_korok] = &root_korok.crates.as_slice() else {
        panic!("Unexpected number of crates");
    };

    // Check the crate korok.
    assert_eq!(
        crate_korok.store.path,
        get_path("nested_modules/crate/src/lib.rs")
    );
    let [ItemKorok::Unsupported(_), ItemKorok::Module(nested_1)] = &crate_korok.items.as_slice()
    else {
        panic!("Unexpected items in lib.rs");
    };
    let [ItemKorok::Module(nested_2), ItemKorok::Unsupported(_), ItemKorok::FileModule(membership)] =
        &nested_1.items.as_slice()
    else {
        panic!("Unexpected items in nested_1 module of lib.rs");
    };
    let [ItemKorok::FileModule(person)] = &nested_2.items.as_slice() else {
        panic!("Unexpected items in nested_2 module of lib.rs");
    };

    // Check the membership module.
    assert_eq!(
        membership.store.path,
        get_path("nested_modules/crate/src/membership.rs")
    );
    let [ItemKorok::Enum(_)] = &membership.items.as_slice() else {
        panic!("Unexpected items in membership.rs");
    };

    // Check the person module.
    assert_eq!(
        person.store.path,
        get_path("nested_modules/crate/src/person.rs")
    );
    let [ItemKorok::Unsupported(_), ItemKorok::Struct(_)] = &person.items.as_slice() else {
        panic!("Unexpected items in person.rs");
    };
}
