use codama_macros::codama;

#[codama(default_value = program("system"))]
pub struct TestWithIdentifier;

#[codama(default_value = program)]
pub struct TestNoParenthesis;

#[codama(default_value = program())]
pub struct TestEmptyParenthesis;

fn main() {}