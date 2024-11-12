#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/into_enum/*.rs");
    t.pass("tests/node/*.rs");
}
