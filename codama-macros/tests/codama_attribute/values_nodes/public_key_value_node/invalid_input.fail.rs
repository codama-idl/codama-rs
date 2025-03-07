use codama_macros::codama;

#[codama(default_value = public_key(42))]
pub struct TestWithInteger;

#[codama(default_value = public_key(banana))]
pub struct TestWithPath;

fn main() {}
