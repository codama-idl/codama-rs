use codama_macros::codama;

#[codama(default_value = pda(name = invalid))]
pub struct TestWithWrongValue;

#[codama(default_value = pda(invalid = "authority"))]
pub struct TestWithWrongKey;

fn main() {}
