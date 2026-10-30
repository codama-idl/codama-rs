use codama_macros::codama;

#[codama(type = fixed_size(42))]
pub struct Test;

#[codama(type = fixed_size(size = 42))]
pub struct TestExplicit;

fn main() {}
