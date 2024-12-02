use crate::get_path;
use codama_stores::RootStore;

#[test]
fn load_single_crate() {
    let path = get_path("single_crate/crate");
    let root_store = RootStore::load_from(&vec![&path]).unwrap();

    // The root store has one crate.
    assert_eq!(root_store.crates.len(), 1);
    let crate_store = &root_store.crates[0];

    // The crate store has the expected items.
    match &crate_store.file.items.as_slice() {
        [syn::Item::Mod(_), syn::Item::Mod(_), syn::Item::Use(_), syn::Item::Use(_)] => {
            assert!(true)
        }
        _ => assert!(false, "Unexpected items in crate"),
    };

    // The crate store has the expected manifest.
    assert!(matches!(
        &crate_store.manifest,
        Some(cargo_toml::Manifest {
            package: Some(_),
            lib: Some(_),
            ..
        })
    ));

    // The crate store has the expected path.
    assert_eq!(crate_store.path, get_path("single_crate/crate/src/lib.rs"));

    // The crate store has 2 file modules.
    assert_eq!(crate_store.file_modules.len(), 2);
    let membership_module = &crate_store.file_modules[0];
    let person_module = &crate_store.file_modules[1];

    // The modules have the expected items.
    match &membership_module.file.items.as_slice() {
        [syn::Item::Enum(_)] => assert!(true),
        _ => assert!(false, "Unexpected items in membership module"),
    };
    match &person_module.file.items.as_slice() {
        [syn::Item::Use(_), syn::Item::Struct(_)] => assert!(true),
        _ => assert!(false, "Unexpected items in person module"),
    };

    // The modules have the correct item indices.
    assert_eq!(membership_module.item_index, 0);
    assert_eq!(person_module.item_index, 1);

    // The modules have the expected paths.
    assert_eq!(
        membership_module.path,
        get_path("single_crate/crate/src/membership.rs")
    );
    assert_eq!(
        person_module.path,
        get_path("single_crate/crate/src/person.rs")
    );

    // The modules have no nested modules.
    assert_eq!(membership_module.file_modules.len(), 0);
    assert_eq!(person_module.file_modules.len(), 0);
}
