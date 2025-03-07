use codama_macros::codama;

#[codama(default_value = sysvar)]
pub struct Test;

#[codama(default_value = sysvar())]
pub struct TestWithParenthesis;

fn main() {}
