use codama_syn_helpers_test_macros::as_path_value;

#[as_path_value(foo = bar(1, 2, 3))]
pub struct Test;

fn main() {}
