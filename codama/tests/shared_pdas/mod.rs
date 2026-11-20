use crate::get_path;
use codama::Codama;

/// Two accounts derived from the same PDA declare it on each variant, and the IDL keeps it once.
#[test]
fn it_keeps_a_pda_shared_by_account_variants_once() {
    let codama = Codama::load(get_path("shared_pdas/crate")).unwrap();
    let idl = codama.get_idl().unwrap();
    let pdas: Vec<_> = idl
        .program
        .pdas
        .iter()
        .map(|pda| pda.identifier.to_string())
        .collect();
    assert_eq!(pdas, ["vault"]);
}

/// Two variants describing a PDA of the same identifier differently fail with the enum that holds them.
#[test]
fn it_fails_when_account_variants_describe_a_pda_differently() {
    let codama = Codama::load(get_path("shared_pdas/conflict")).unwrap();
    assert_eq!(
        codama.get_idl().unwrap_err().to_string(),
        "Two different `pdaNode` share the identifier `vault` in enum `MyAccounts`"
    );
}
