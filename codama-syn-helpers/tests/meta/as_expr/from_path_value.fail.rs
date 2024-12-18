use codama_syn_helpers_test_macros::as_expr;

#[as_expr(foo = bar(1, 2, 3))]
pub struct Test;

fn main() {}
