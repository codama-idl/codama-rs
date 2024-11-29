use codama_errors::CodamaError;
use std::io::ErrorKind::NotFound;

#[test]
fn from_io_error() {
    let error: CodamaError = std::io::Error::new(NotFound, "Could not find file `foo.rs`").into();
    assert!(matches!(error, CodamaError::Filesystem(_)));
}

#[test]
fn display() {
    let error: CodamaError = std::io::Error::new(NotFound, "Could not find file `foo.rs`").into();
    assert_eq!(error.to_string(), "IO error: Could not find file `foo.rs`");
}
