#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/simple.rs");
    t.pass("tests/generic.rs");
    t.pass("tests/boxed.rs");
}
