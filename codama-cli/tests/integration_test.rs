use predicates::prelude::*;
use std::fs;

// Helper function to create CLI command for testing.
// Uses Command::cargo_bin which is deprecated, but this is the recommended
// approach in assert_cmd's documentation until cargo_bin_cmd! macro is stable.
// See: https://github.com/assert-rs/assert_cmd/issues/139
#[allow(deprecated)]
fn get_cli_command() -> assert_cmd::Command {
    assert_cmd::Command::cargo_bin("codama-rs").unwrap()
}

#[test]
fn test_generate_idl_simple_account() {
    let mut cmd = get_cli_command();

    let crate_path = fs::canonicalize("tests/fixtures/test-simple-account").unwrap();

    cmd.arg("generate-idl")
        .arg(crate_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""kind":"rootNode""#))
        .stdout(predicate::str::contains(r#""name":"testSimpleAccount""#));
}

#[test]
fn test_generate_idl_current_dir_fail() {
    let mut cmd = get_cli_command();
    cmd.arg("generate-idl")
        .arg("non_existent_path")
        .assert()
        .failure();
}

#[test]
fn test_generate_idl_with_pretty_flag() {
    let mut cmd = get_cli_command();
    let crate_path = fs::canonicalize("tests/fixtures/test-simple-account").unwrap();

    cmd.arg("generate-idl")
        .arg(crate_path)
        .arg("--pretty")
        .assert()
        .success()
        .stdout(predicate::str::contains("{\n  \"kind\": \"rootNode\""));
}

#[test]
fn test_generate_idl_with_output_flag() {
    let temp_dir = std::env::temp_dir();
    let output_file = temp_dir.join(format!("test_idl_{:?}.json", std::thread::current().id()));

    let _ = fs::remove_file(&output_file);

    let mut cmd = get_cli_command();
    let crate_path = fs::canonicalize("tests/fixtures/test-simple-account").unwrap();

    cmd.arg("generate-idl")
        .arg(crate_path)
        .arg("--output")
        .arg(&output_file)
        .assert()
        .success();

    let content = fs::read_to_string(&output_file).unwrap();
    assert!(content.contains(r#""kind":"rootNode""#));

    fs::remove_file(&output_file).unwrap();
}

#[test]
fn test_version_flag() {
    let mut cmd = get_cli_command();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}
