use codama_macros::codama;

#[codama(seed)]
pub struct EmptyTest;

#[codama(seed())]
pub struct EmptyTestWithBraces;

#[codama(seed(type = integer(u8)))]
pub struct TestWithTypeOnly;

fn main() {}
