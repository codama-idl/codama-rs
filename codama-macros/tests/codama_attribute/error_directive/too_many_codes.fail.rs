use codama_macros::codama;

#[codama(error(42, 100))]
pub struct Test;

#[codama(error(code = 42, code = 100))]
pub struct TestExplicit;

fn main() {}
