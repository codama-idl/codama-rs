use codama_macros::codama;

#[codama(type = field(number(u32)))]
pub struct Test;

#[codama(type = field(type = number(u32)))]
pub struct TestExplicit;

fn main() {}
