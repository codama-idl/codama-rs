use codama_macros::codama;

#[codama(pda)]
pub struct Test;

#[codama(pda())]
pub struct TestWithBraces;

fn main() {}
