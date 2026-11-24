use crate::get_path;
use codama::Codama;

/// A `program(...)` directive on a `mod external;` declaration, or as an inner attribute at the top of
/// the file, moves the items of the file into that program, like on an inline module.
#[test]
fn it_applies_program_directives_to_file_modules() {
    let codama = Codama::load(get_path("program_directives/crate")).unwrap();
    let idl = codama.get_idl().unwrap();
    let accounts = |program: &codama::ProgramNode| {
        program
            .accounts
            .iter()
            .map(|account| account.identifier.to_string())
            .collect::<Vec<_>>()
    };
    assert_eq!(idl.program.identifier.as_ref(), "program_directives");
    assert_eq!(accounts(&idl.program), ["config"]);
    let additional: Vec<_> = idl
        .additional_programs
        .iter()
        .map(|program| (program.identifier.to_string(), accounts(program)))
        .collect();
    assert_eq!(
        additional,
        [
            ("external".to_string(), vec!["vault".to_string()]),
            ("declared".to_string(), vec!["ticket".to_string()]),
            ("other".to_string(), vec!["escrow".to_string()]),
        ]
    );
}

/// A `#![codama(program(...))]` at the top of the crate root names the main program.
#[test]
fn it_applies_program_directives_to_the_crate_root() {
    let codama = Codama::load(get_path("program_directives/root")).unwrap();
    let idl = codama.get_idl().unwrap();
    assert_eq!(idl.program.identifier.as_ref(), "named");
    assert_eq!(
        idl.program.public_key,
        "NamedProg11111111111111111111111111111111111"
    );
    assert_eq!(idl.program.accounts.len(), 1);
    assert!(idl.additional_programs.is_empty());
}
