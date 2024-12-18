use codama::codama;

#[codama(type = fixed_size(boolean, 42, number(u32)))]
pub struct Test;

#[codama(type = fixed_size(type = boolean, size = 42, type = number(u32)))]
pub struct TestExplicit;

fn main() {}
