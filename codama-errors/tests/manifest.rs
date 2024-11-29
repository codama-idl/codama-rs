use codama_errors::CodamaError;

#[test]
fn from_manifest_error() {
    let error: CodamaError = cargo_toml::Error::Other("Your Cargo.toml file is invalid").into();
    assert!(matches!(error, CodamaError::Manifest(_)));
}

#[test]
fn display() {
    let error: CodamaError = cargo_toml::Error::Other("Your Cargo.toml file is invalid").into();
    assert_eq!(
        error.to_string(),
        "Could not parse Cargo.toml files: Your Cargo.toml file is invalid"
    );
}
