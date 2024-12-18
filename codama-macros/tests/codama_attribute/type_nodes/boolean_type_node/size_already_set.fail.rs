use codama::codama;

#[codama(type = boolean(number(u32), number(u32)))]
pub struct Test;

#[codama(type = boolean(size = number(u32), size = number(u32)))]
pub struct TestExplicit;

fn main() {}
