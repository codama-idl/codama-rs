use codama_macros::codama;

#[codama(default_value = account_bump(42))]
pub struct TestWithInteger;

#[codama(default_value = account_bump(banana))]
pub struct TestWithPath;

#[codama(default_value = account_bump(banana = "escrow"))]
pub struct TestWithWrongKey;

fn main() {}
