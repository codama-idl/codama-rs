use codama_macros::codama;

#[codama(default_value = data(42))]
pub struct TestWithInteger;

#[codama(default_value = data(banana))]
pub struct TestWithPath;

#[codama(default_value = data(name = "amount"))]
pub struct TestWithKey;

#[codama(default_value = data("a..b"))]
pub struct TestWithInvalidPath;

fn main() {}
