use codama_syn_helpers_test_macros::as_verbatim;

#[as_verbatim(foo(1, 2, 3))]
pub struct Test;

#[as_verbatim(foo = (1, 2, 3))]
pub struct TestWithEq;

fn main() {}
