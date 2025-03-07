use codama_macros::codama;

#[codama(default_value = public_key)]
pub struct Test;

#[codama(default_value = public_key())]
pub struct TestWithParenthesis;

fn main() {}
