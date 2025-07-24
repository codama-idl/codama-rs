use codama_macros::codama;

#[codama(default_value = program)]
pub struct Test;

#[codama(default_value = program())]
pub struct TestWithParenthesis;

fn main() {}