#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/into_enum/*.rs");
    t.pass("tests/node/*.rs");
    t.pass("tests/node_union/*.rs");
    t.pass("tests/registered_nodes/*.pass.rs");
    t.compile_fail("tests/registered_nodes/*.fail.rs");
}
