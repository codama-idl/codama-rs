use codama_macros::codama;

#[codama(default_value = account_bump)]
pub struct Test;

#[codama(default_value = account_bump())]
pub struct TestWithBraces;

fn main() {}
