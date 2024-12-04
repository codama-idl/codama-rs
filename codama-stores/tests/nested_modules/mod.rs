use crate::get_path;
use codama_stores::RootStore;

#[test]
fn load_nested_modules() {
    let path = get_path("nested_modules/crate");
    let root_store = RootStore::load_from(&[&path]).unwrap();

    // The root store has one crate.
    assert_eq!(root_store.crates.len(), 1);
    let crate_store = &root_store.crates[0];

    // The file modules are exported using depth-first search.
    let [person, membership] = &crate_store.file_modules.as_slice() else {
        panic!("Unexpected file modules in crate store");
    };
    assert_eq!(person.path, get_path("nested_modules/crate/src/person.rs"));
    assert_eq!(
        membership.path,
        get_path("nested_modules/crate/src/membership.rs")
    );
}
