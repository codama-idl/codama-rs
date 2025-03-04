use codama_macros::codama;

#[codama(type = number(le))]
pub struct Test;

#[codama(type = number(endian = le))]
pub struct TestExplicit;

fn main() {}
