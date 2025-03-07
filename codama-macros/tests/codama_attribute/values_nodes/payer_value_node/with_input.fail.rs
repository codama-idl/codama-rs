use codama_macros::codama;

#[codama(default_value = payer(42))]
pub struct TestWithInteger;

#[codama(default_value = payer(banana))]
pub struct TestWithPath;

fn main() {}
