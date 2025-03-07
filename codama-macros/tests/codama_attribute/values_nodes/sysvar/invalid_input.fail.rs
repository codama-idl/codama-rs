use codama_macros::codama;

#[codama(default_value = sysvar(42))]
pub struct TestWithInteger;

#[codama(default_value = sysvar(banana))]
pub struct TestWithPath;

fn main() {}
