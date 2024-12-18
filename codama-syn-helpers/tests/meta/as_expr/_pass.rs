use codama_syn_helpers_test_macros::as_expr;

#[as_expr("hello")]
pub struct Test;

#[as_expr(foo)]
pub struct TestFromPath;

fn main() {}
