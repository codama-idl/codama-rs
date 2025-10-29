use codama_macros::codama;

#[codama(default_value = argument(42))]
pub struct TestWithInteger;

#[codama(default_value = argument(banana))]
pub struct TestWithPath;

#[codama(default_value = argument(banana = "authority"))]
pub struct TestWithWrongKey;

fn main() {}
