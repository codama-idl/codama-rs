use crate::get_path;
use codama::Codama;

/// Two modules of a file defining different items with the same identifier fail with the module
/// and the file that holds them.
#[test]
fn it_names_the_file_of_conflicting_module_items() {
    let codama = Codama::load(get_path("module_conflicts/crate")).unwrap();
    let error = codama.get_idl().unwrap_err();
    let file = get_path("module_conflicts/crate/src/state.rs");
    assert_eq!(
        error.to_string(),
        format!(
            "Two different `definedTypeNode` share the identifier `config` in module `state` ({})",
            file.display()
        )
    );
}
