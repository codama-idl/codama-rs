use codama_macros::codama;

#[codama(default_value = argument)]
pub struct Test;

#[codama(default_value = argument())]
pub struct TestWithBraces;

fn main() {}
