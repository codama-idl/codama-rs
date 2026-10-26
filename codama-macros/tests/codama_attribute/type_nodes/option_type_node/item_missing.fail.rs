use codama_macros::codama;

#[codama(type = option(fixed))]
pub struct Test;

#[codama(type = option(prefix = integer(u16)))]
pub struct TestExplicit;

fn main() {}
