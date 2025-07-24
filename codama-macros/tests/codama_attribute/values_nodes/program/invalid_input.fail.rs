use codama_macros::codama;

#[codama(default_value = program(42))]
pub struct TestWithInteger;

#[codama(default_value = program(banana))]
pub struct TestWithPath;

fn main() {}