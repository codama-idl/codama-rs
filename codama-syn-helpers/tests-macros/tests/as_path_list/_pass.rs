use codama_syn_helpers_test_macros::as_path_list;

#[as_path_list(foo(1, 2, 3))]
pub struct Test;

#[as_path_list(foo = (1, 2, 3))]
pub struct TestWithEq;

fn main() {}
