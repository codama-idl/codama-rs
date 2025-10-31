use codama_macros::codama;

#[codama(default_value = pda("token", [42, invalid, "invalid"]))]
pub struct TestWithUnrecognizedSeeds;

#[codama(default_value = pda("token", [seed(42), seed(value = 43)]))]
pub struct TestWithNamelessSeeds;

#[codama(default_value = pda("token", [seed("mint"), seed("owner")]))]
pub struct TestWithValuelessSeeds;

fn main() {}
