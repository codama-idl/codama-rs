use codama_macros::codama;

#[codama(default_value = payer)]
pub struct Test;

#[codama(default_value = payer())]
pub struct TestWithParenthesis;

fn main() {}
