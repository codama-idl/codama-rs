use codama_macros::codama;

#[codama(default_value = account)]
pub struct Test;

#[codama(default_value = account())]
pub struct TestWithBraces;

fn main() {}
