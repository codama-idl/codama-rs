use codama_errors::CodamaError;

#[test]
fn display_names_the_path() {
    let error = CodamaError::Manifest {
        path: "programs/token/Cargo.toml".into(),
        source: cargo_toml::Error::Other("no crate path found"),
    };
    assert_eq!(
        error.to_string(),
        "Could not parse the manifest `programs/token/Cargo.toml`: no crate path found"
    );
}
