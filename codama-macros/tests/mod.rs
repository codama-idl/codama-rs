#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/**/_pass.rs");
    t.pass("tests/**/*.pass.rs");
    t.compile_fail("tests/**/*.fail.rs");
}
