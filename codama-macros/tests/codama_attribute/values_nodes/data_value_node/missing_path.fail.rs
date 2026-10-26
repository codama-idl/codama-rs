use codama_macros::codama;

#[codama(default_value = data)]
pub struct Test;

#[codama(default_value = data())]
pub struct TestWithBraces;

fn main() {}
