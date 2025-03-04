use codama_macros::codama;

#[codama(type = fixed_size(42, boolean, 100))]
pub struct Test;

#[codama(type = fixed_size(size = 42, type = boolean, size = 100))]
pub struct TestExplicit;

fn main() {}
