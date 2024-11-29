use codama_errors::CodamaError;

#[test]
fn display() {
    let error = CodamaError::InvalidNodeConversion {
        from: "registeredTypeNode".to_string(),
        into: "typeNode".to_string(),
    };
    assert_eq!(
        error.to_string(),
        "Could not convert node from `registeredTypeNode` into `typeNode`"
    );
}
