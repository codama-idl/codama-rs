use codama_errors::CodamaError;
use std::io::ErrorKind::NotFound;

#[test]
fn display_names_the_path() {
    let error = CodamaError::Filesystem {
        path: "programs/token/src/lib.rs".into(),
        source: std::io::Error::new(NotFound, "No such file or directory"),
    };
    assert_eq!(
        error.to_string(),
        "Could not read `programs/token/src/lib.rs`: No such file or directory"
    );
}
