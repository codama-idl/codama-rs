use codama_syn_helpers_test_macros::as_verbatim;

#[as_verbatim(foo = bar(1, 2, 3))]
pub struct Test;

fn main() {}
