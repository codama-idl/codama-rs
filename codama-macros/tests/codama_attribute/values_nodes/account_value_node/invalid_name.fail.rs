use codama_macros::codama;

#[codama(default_value = account(42))]
pub struct TestWithInteger;

#[codama(default_value = account(banana))]
pub struct TestWithPath;

#[codama(default_value = account(banana = "authority"))]
pub struct TestWithWrongKey;

fn main() {}
